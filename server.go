package main

/*
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"github.com/Manta-Network/Manta-Singer/utils"
	"github.com/labstack/echo/v4"
	"github.com/wailsapp/wails/v2"
	"io/ioutil"
	"log"
	"net/http"
)

type Svr struct {
	runtime     *wails.Runtime
	engine      *echo.Echo
	unlockQueue chan struct{}
}

func NewSvr() *Svr {
	return &Svr{
		engine: echo.New(),
		// 默认只有一个解锁队列
		unlockQueue: make(chan struct{}, 1),
	}
}

func (s *Svr) RegisterRoutes() {
	s.engine.GET("/heartbeat", heartbeat)
	group := s.engine.Group("/auth")
	group.Use(func(next echo.HandlerFunc) echo.HandlerFunc {
		return func(context echo.Context) error {
			if !utils.AccountCreated() {
				s.runtime.Events.Emit("manta.browser.openCreate")
				s.runtime.Window.Show()
				return nil
			} else {
				s.runtime.Events.Emit("manta.browser.openUnlock")
				s.runtime.Window.Show()
				<-s.unlockQueue
				// 关闭window
				s.runtime.Window.Hide()
				return next(context)
			}
		}
	})
	group.POST("/generateMintData", s.generateMintData)
	group.POST("/generatePrivateTransferData", s.generatePrivateTransferData)
	group.POST("/generateReclaimData", s.generateReclaimData)
	group.POST("/deriveShieldedAddress", s.deriveShieldedAddress)
	group.POST("/generateAsset", s.generateAsset)
	group.POST("/recoverAccount", s.recoverAccount)
}

func (s *Svr) Start(runtime *wails.Runtime, addr string) error {
	s.runtime = runtime
	s.runtime.Events.On("manta.server.onUnlocked", func(optionalData ...interface{}) {
		s.unlockQueue <- struct{}{}
	})
	return s.engine.Start(addr)
}

func heartbeat(ctx echo.Context) error {
	return nil
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
	ret := C.generate_private_transfer_data((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	ret := C.generate_reclaim_data((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	ret := C.derive_shielded_address((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	ret := C.generate_mint_data((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	ret := C.generate_asset((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
	ret := C.recover_account((*C.uchar)(&bytes[0]), C.size_t(len(bytes)), &outBufferRef, &outLen)
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
