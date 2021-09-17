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

func (s *Svr) Start(runtime *wails.Runtime, addr string) error {
	s.runtime = runtime
	return s.engine.Start(addr)
}

func heartbeat(ctx echo.Context) error {
	return nil
}

func (s *Svr) awaitUnlock(ctx echo.Context, onUnlock func(echo.Context) error) error {
	s.runtime.Window.Show()
	s.runtime.Events.Emit("manta.browser.openUnlock")

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
		return onUnlock(ctx)
	} else {
		return ctx.JSON(http.StatusUnauthorized, "Transaction rejected by user")
	}
}

func (s *Svr) requestGenerateReclaimData(ctx echo.Context) error {
	// s.awaitSignIn()
	onUnlock := s.generateReclaimData
	return s.awaitUnlock(ctx, onUnlock)
}

func (s *Svr) requestGeneratePrivateTransferData(ctx echo.Context) error {
	// s.awaitSignIn()
	onUnlock := s.generatePrivateTransferData
	return s.awaitUnlock(ctx, onUnlock)
}

func (s *Svr) generatePrivateTransferData(ctx echo.Context) error {
	appVersion := ctx.QueryParam("app_version")
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
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.batch_generate_private_transfer_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"private_transfer_data": C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version":        version,
		"app_version":           appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

func (s *Svr) generateReclaimData(ctx echo.Context) error {
	appVersion := ctx.QueryParam("app_version")
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	if len(bytes) == 0 {
		return echo.ErrBadRequest
	}
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	var outLen C.size_t
	ret := C.batch_generate_reclaim_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"reclaim_data":   C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version": version,
		"app_version":    appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}


func (s *Svr) deriveShieldedAddress(ctx echo.Context) error {
	// s.awaitSignIn()
	appVersion := ctx.QueryParam("app_version")
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
		"app_version":    appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

func (s *Svr) generateMintData(ctx echo.Context) error {
	// s.awaitSignIn()
	appVersion := ctx.QueryParam("app_version")
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
		"app_version":    appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

func (s *Svr) generateAsset(ctx echo.Context) error {
	// s.awaitSignIn()
	appVersion := ctx.QueryParam("app_version")
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
		"app_version":    appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

func (s *Svr) recoverAccount(ctx echo.Context) error {
	// s.awaitSignIn()
	appVersion := ctx.QueryParam("app_version")
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
		"app_version":       appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}
