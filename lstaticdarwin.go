// +build darwin

package main

/*
#cgo LDFLAGS: -L./lib -lzkp
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"github.com/labstack/echo/v4"
	"io/ioutil"
	"log"
	"net/http"
	"unsafe"
)

func generateTransferZKP(ctx echo.Context) error {
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
	ret := C.generate_transfer(C.CString(appVersion), (*C.char)(unsafe.Pointer(&bytes[0])), C.size_t(len(bytes)), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"transfer_zkp":   C.GoString(outBufferRef),
		"daemon_version": DaemonVersion,
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

func generateReclaimZKP(ctx echo.Context) error {
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
		"daemon_version": DaemonVersion,
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
