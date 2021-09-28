package main

/*
#cgo LDFLAGS: -L./lib -lzkp
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"io/ioutil"
	"log"
	"net/http"

	"github.com/Manta-Network/Manta-Singer/utils"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"github.com/wailsapp/wails/v2"
)

type Svr struct {
	rootSeed   *[64]byte
	userIsSignedIn *bool
	runtime     *wails.Runtime
	engine      *echo.Echo
	unlockQueue chan struct{}
}

// For all requests, wait indefinitely until the user has created an account
// Then wait indefinitely until the user has signed in through Manta Signer UI
// and thus loaded the root seed into memory
func (s *Svr) awaitSignIn(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c echo.Context) error {
		if (!utils.AccountCreated()) {
			s.runtime.Window.Show()
			s.runtime.Events.Emit("manta.browser.openCreateAccount")
			for loop := true; loop; loop = !utils.AccountCreated() {}
		}
		if (!*s.userIsSignedIn) {
			s.runtime.Window.Show()
			s.runtime.Events.Emit("manta.browser.openSignIn")
			for loop := true; loop; loop = !*s.userIsSignedIn {}
		}
		return next(c)
	}
}

// Constructs Manta Signer's daemon server
func NewSvr(rootSeed *[64]byte, userIsSignedIn *bool) *Svr {
	server := Svr{
		rootSeed: rootSeed,
		userIsSignedIn: userIsSignedIn,
		engine: echo.New(),
	}
	server.engine.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:8000", "https://manta.network"},
		AllowMethods: []string{http.MethodGet, http.MethodPost},
	}))
	server.engine.Use(server.awaitSignIn)
	return &server
}

// Defines the routes that Manta Signer's daemon server exposes
func (s *Svr) RegisterRoutes() {
	// Not sensitive
	s.engine.GET("/heartbeat", heartbeat)
	s.engine.POST("/generateMintData", s.generateMintData)
	s.engine.POST("/generateAsset", s.generateAsset)
	s.engine.POST("/deriveShieldedAddress", s.deriveShieldedAddress)
	s.engine.POST("/recoverAccount", s.recoverAccount)
	// Sensitive
	s.engine.POST("/requestGenerateReclaimData", s.requestGenerateReclaimData)
	s.engine.POST("/requestGeneratePrivateTransferData", s.requestGeneratePrivateTransferData)
}

// Starts Manta Signer's daemon server
func (s *Svr) Start(runtime *wails.Runtime, addr string) error {
	s.runtime = runtime
	return s.engine.Start(addr)
}

// Allows client to verify that Manta Signer's daemon is running
func heartbeat(ctx echo.Context) error {
	return nil
}

// For use in sensitive endpoints;
// Prompts the user to approve an incoming transaction in Signer UI,
// then waits indefinitely for the user to either decline the transaction,
// or enter the account password and approve the transaction. On approval, the
// `onUnlock` function will generate the transaction payload and return it to the client
func (s *Svr) awaitAuthorizeTransaction(
	ctx echo.Context,
	onUnlock func(ctx echo.Context, bytes []byte) error,
	bytes []byte,
	transactionType string,
	) error {

	transactionBatchSummary := getTransactionBatchSummary(bytes, transactionType)
	s.runtime.Window.Show()
	s.runtime.Events.Emit(
		"manta.browser.openAuthorizeTransaction",
		transactionBatchSummary.transactionType,
		transactionBatchSummary.value,
		transactionBatchSummary.denomination,
		transactionBatchSummary.recipient,
	)

	unlockPopupResolved := false
	var success bool
	s.runtime.Events.On("manta.server.onUnlockFail", func(optionalData ...interface{}) {
		unlockPopupResolved = true
		success = false
	})
	s.runtime.Events.On("manta.server.onUnlockSuccess", func(optionalData ...interface{}) {
		unlockPopupResolved = true
		success = true
	})

	for loop := true; loop; loop = !unlockPopupResolved {}
	if success {
		return onUnlock(ctx, bytes)
	} else {
		return ctx.JSON(http.StatusUnauthorized, "Transaction rejected by user")
	}
}

// Returns a reclaim payload to the client if the user inputs a password and
// approves the reclaim through Manta Signer's UI; see `awaitUnlock` above
func (s *Svr) requestGenerateReclaimData(ctx echo.Context) error {
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
		return err
	}
	if len(bytes) == 0 {
		return echo.ErrBadRequest
	}

	onUnlock := s.generateReclaimData
	return s.awaitAuthorizeTransaction(ctx, onUnlock, bytes, "Reclaim")
}

// Returns a reclaim payload to the client if the user inputs a password and approves the private
// transfer through Manta Signer's UI; see `awaitUnlock` above
func (s *Svr) requestGeneratePrivateTransferData(ctx echo.Context) error {
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
		return err
	}
	if len(bytes) == 0 {
		return echo.ErrBadRequest
	}

	onUnlock := s.generatePrivateTransferData
	return s.awaitAuthorizeTransaction(ctx, onUnlock, bytes, "Private transfer")
}

// Generates a private transfer payload (Go -> C -> Rust) and returns the
// payload plus relevant metadata to the client
// Payload generation logic lives in rust code, which this function only wraps
// see: `lib/zkp`
func (s *Svr) generatePrivateTransferData(ctx echo.Context, bytes []byte) error {
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.batch_generate_private_transfer_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"private_transfer_data": C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version":        version,
		"app_version":           ctx.QueryParam("app_version"),
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

// Generates a reclaim payload (Go -> C -> Rust) and returns the payload plus
// relevant metadata to the client
// Payload generation logic lives in rust code, which this function only wraps
// see: `lib/zkp`
func (s *Svr) generateReclaimData(ctx echo.Context, bytes []byte) error {
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.batch_generate_reclaim_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"reclaim_data":   C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version": version,
		"app_version":    ctx.QueryParam("app_version"),
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

// Generates a shielded address and returns the payload plus relevant metadata to the client
// Payload generation logic lives in rust code, which this function only wraps
// see: `lib/zkp`
func (s *Svr) deriveShieldedAddress(ctx echo.Context) error {
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.derive_shielded_address((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"address":        C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version": version,
		"app_version":    ctx.QueryParam("app_version"),
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

// Generates a mint payload and returns the payload plus relevant metadata to the client
// Payload generation logic lives in rust code, which this function only wraps
// see: `lib/zkp`
func (s *Svr) generateMintData(ctx echo.Context) error {
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.generate_mint_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"mint_data":      C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version": version,
		"app_version":    ctx.QueryParam("app_version"),
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

// Generates a manta asset stripped of sensitive data and returns the payload
// plus relevant metadata to the client
// Payload generation logic lives in rust code, which this function only wraps
// see: `lib/zkp`
func (s *Svr) generateAsset(ctx echo.Context) error {
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.generate_asset((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"asset":          C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version": version,
		"app_version":    ctx.QueryParam("app_version"),
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

// Generates a list of spendable assets owned by the root seed  and returns the
// assets plus relevant metadata to the client
// Payload generation logic lives in rust code, which this function only wraps
// see: `lib/zkp`
func (s *Svr) recoverAccount(ctx echo.Context) error {
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.recover_account((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"length":            C.int(outLen),
		"recovered_account": C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version":    version,
		"app_version":       ctx.QueryParam("app_version"),
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}
