// +build windows

package main

/*
#cgo LDFLAGS: -L./lib/windows -lzkp -ladvapi32 -lws2_32 -luserenv
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"encoding/json"
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

func deriveShieldedAddress(ctx echo.Context) error {
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

func generateAsset(ctx echo.Context) error {
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
