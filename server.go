package main

/*
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"encoding/json"
	"github.com/labstack/echo/v4"
	"github.com/wailsapp/wails/v2"
	"io/ioutil"
	"log"
	"net/http"
	"unsafe"
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
			// todo 判断是否创建account
			if C.account_created() == 0 {
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
	group.POST("/generateTransferZKP", s.generateTransferZKP)
	group.POST("/generateReclaimZKP", s.generateReclaimZKP)
	group.POST("/deriveShieldedAddress", s.deriveShieldedAddress)
	group.POST("/generateAsset", s.generateAsset)
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

func (s *Svr) generateTransferZKP(ctx echo.Context) error {
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
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	ret := C.generate_transfer(C.CString(appVersion), (*C.char)(unsafe.Pointer(&bytes[0])), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"transfer_zkp":   C.GoString(outBufferRef),
		"daemon_version": version,
		"app_version":    appVersion,
	}
	C.free(unsafe.Pointer(outBufferRef))
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
	return nil
}

func (s *Svr) generateReclaimZKP(ctx echo.Context) error {
	appVersion := ctx.QueryParam("app_version")
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	ret := C.generate_reclaim(C.CString(appVersion), (*C.char)(unsafe.Pointer(&bytes[0])), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"reclaim_zkp":    C.GoString(outBufferRef),
		"daemon_version": version,
		"app_version":    appVersion,
	}
	C.free(unsafe.Pointer(outBufferRef))
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
	return nil
}

type deriveShieldedAddressArgs struct {
	Path    string `json:"path"`
	AssetId int    `json:"assetId"`
}

func (s *Svr) deriveShieldedAddress(ctx echo.Context) error {
	args := &deriveShieldedAddressArgs{}
	if err := ctx.Bind(args); err != nil {
		log.Fatal(err.Error())
		return err
	}
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	ret := C.derive_shielded_address(C.CString(args.Path), C.int(args.AssetId), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"address":        C.GoString(outBufferRef),
		"daemon_version": version,
		"app_version":    "0.0",
	}
	C.free(unsafe.Pointer(outBufferRef))
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}

func (s *Svr) generateAsset(ctx echo.Context) error {
	jsonMap := make(map[string]interface{})
	err := json.NewDecoder(ctx.Request().Body).Decode(&jsonMap)
	if err != nil {
		return err
	}
	path := jsonMap["path"].(string)
	assetId := int(jsonMap["assetId"].(float64))
	value := jsonMap["value"].(string)

	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	ret := C.generate_asset(C.int(assetId), C.CString(value), C.CString(path), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"address":        C.GoString(outBufferRef),
		"daemon_version": version,
		"app_version":    "0.0",
	}
	C.free(unsafe.Pointer(outBufferRef))
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
}
