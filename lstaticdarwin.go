// +build darwin

package main

/*
#cgo LDFLAGS: -L./lib -lzkp
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"encoding/json"
	"io/ioutil"
	"log"
	"net/http"
	"unsafe"

	"github.com/labstack/echo/v4"
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
	log.Print(body)
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

func deriveShieldedAddress(ctx echo.Context) error {
	json_map := make(map[string]interface{})
	err := json.NewDecoder(ctx.Request().Body).Decode(&json_map)
	if err != nil {
		return err
	}
	path := json_map["path"].(string)
	assetId := int(json_map["assetId"].(float64))

	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	ret := C.derive_shielded_address(C.CString(path), C.int(assetId), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"address":        C.GoString(outBufferRef),
		"daemon_version": DaemonVersion,
		"app_version":    "0.0",
	}
	C.free(unsafe.Pointer(outBufferRef))
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
	return nil
}

func generateAsset(ctx echo.Context) error {
	json_map := make(map[string]interface{})
	err := json.NewDecoder(ctx.Request().Body).Decode(&json_map)
	if err != nil {
		return err
	}
	path := json_map["path"].(string)
	assetId := int(json_map["assetId"].(float64))
	value := json_map["value"].(string)

	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	ret := C.generate_asset(C.int(assetId), C.CString(value), C.CString(path), &outBufferRef, &outLen)
	message := map[string]interface{}{
		"address":        C.GoString(outBufferRef),
		"daemon_version": DaemonVersion,
		"app_version":    "0.0",
	}
	C.free(unsafe.Pointer(outBufferRef))
	if ret == 0 {
		return ctx.JSON(http.StatusOK, message)
	} else {
		return ctx.JSON(http.StatusInternalServerError, message)
	}
	return nil

}
