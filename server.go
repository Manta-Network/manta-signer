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

func NewSvr(rootSeed *[64]byte, userIsSignedIn *bool) *Svr {
	server := Svr{
		rootSeed: rootSeed,
		userIsSignedIn: userIsSignedIn,
		engine: echo.New(),
	}
	server.engine.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"http://localhost:8000"},
		AllowMethods: []string{http.MethodGet, http.MethodPut, http.MethodPost, http.MethodDelete},
	}))
	return &server
}

func (s *Svr) RegisterRoutes() {
	println("account created?", utils.AccountCreated())
	// Not sensitive
	s.engine.GET("/heartbeat", heartbeat)
	s.engine.POST("/generateMintData", s.generateMintData)
	s.engine.POST("/generateAsset", s.generateAsset)
	s.engine.POST("/deriveShieldedAddress", s.deriveShieldedAddress)
	s.engine.POST("/recoverAccount", s.recoverAccount)

	// Sensitive
	s.engine.POST("/requestGenerateReclaimData", s.requestGenerateReclaimData)
	s.engine.POST("/requestGeneratePrivateTransferData", s.requestGeneratePrivateTransferData)


	// group := s.engine.Group("/auth")
	// group.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
	// 	return func(context echo.Context) error {
	// 		if !utils.AccountCreated() {
	// 			s.runtime.Events.Emit("manta.browser.openCreate")
	// 			s.runtime.Window.Show()
	// 			return nil
	// 		} else {
	// 			s.runtime.Events.Emit("manta.browser.openUnlock")
	// 			s.runtime.Window.Show()
	// 			<-s.unlockQueue
	// 			// 关闭window
	// 			s.runtime.Window.Hide()
	// 			return next(context)
	// 		}
	// 	}
	// })
}



func (s *Svr) Start(runtime *wails.Runtime, addr string) error {
	s.runtime = runtime
	return s.engine.Start(addr)
}

func heartbeat(ctx echo.Context) error {
	return nil
}

func (s *Svr) awaitSignIn() {
	s.runtime.Window.Show()
	s.runtime.Events.Emit("manta.browser.openUnlock")
	for loop := true; loop; loop = !*s.userIsSignedIn {}
}

func (s *Svr) requestGenerateReclaimData(ctx echo.Context) error {
	s.runtime.Window.Show()
	s.runtime.Events.Emit("manta.browser.openUnlock")

	popupResolved := false
	var success bool
	s.runtime.Events.On("manta.server.onUnlockFail", func(optionalData ...interface{}) {
		popupResolved = true
		success = false
	})
	s.runtime.Events.On("manta.server.onUnlockSuccess", func(optionalData ...interface{}) {
		popupResolved = true
		success = true
	})

	for loop := true; loop; loop = !popupResolved {}
	if success {
		return s.generateReclaimData(ctx)
	} else {
		return ctx.JSON(http.StatusUnauthorized, "Transaction rejected by user")
	}
}

func (s *Svr) requestGeneratePrivateTransferData(ctx echo.Context) error {
	s.runtime.Window.Show()
	s.runtime.Events.Emit("manta.browser.openUnlock")
	popupResolved := false
	var success bool
	s.runtime.Events.On("manta.server.onUnlockFail", func(optionalData ...interface{}) {
		popupResolved = true
		success = false
	})
	s.runtime.Events.On("manta.server.onUnlockSuccess", func(optionalData ...interface{}) {
		popupResolved = true
		success = true
	})
	for loop := true; loop; loop = !popupResolved {}
	if success {
		println(4)
		return s.generatePrivateTransferData(ctx)
	} else {
		println(5)
		return ctx.JSON(http.StatusUnauthorized, "Transaction rejected by user")
	}
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
	ret := C.generate_private_transfer_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"private_transfer_data": C.GoBytes(outBufferRef, C.int(outLen)),
		"daemon_version":        version,
		"app_version":           appVersion,
	}
	C.free(outBufferRef)
	if ret == 0 {
		println(message)
		return ctx.JSON(http.StatusOK, message)
	} else {
		println(message)
		return ctx.JSON(http.StatusInternalServerError, message)
	}
	return nil
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
	ret := C.generate_reclaim_data((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	return nil
}


func (s *Svr) deriveShieldedAddress(ctx echo.Context) error {
	s.awaitSignIn()
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
	ret := C.debug_derive_shielded_address((*C.uchar)(&(*s.rootSeed)[0]), (*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	return nil
}

func (s *Svr) generateMintData(ctx echo.Context) error {
	s.awaitSignIn()
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
	return nil
}

func (s *Svr) generateAsset(ctx echo.Context) error {
	s.awaitSignIn()
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
	return nil
}

func (s *Svr) recoverAccount(ctx echo.Context) error {
	s.awaitSignIn()
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
	return nil
}
