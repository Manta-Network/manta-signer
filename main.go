package main

/*
#cgo LDFLAGS: -L./lib -lhello
#include "./lib/hello.h"
#include <stdlib.h>
*/
import "C"
import (
	"fmt"
	"github.com/kardianos/service"
	"github.com/labstack/echo/v4"
	"io/ioutil"
	"log"
	"net/http"
	"unsafe"
)

const (
	DaemonVersion = "0.1.0"
)

var logger service.Logger

type program struct{}

func (p program) Start(s service.Service) error {
	go p.run()
	return nil
}

func (p program) run() {
	e := echo.New()
	e.POST("/heartbeat", heartbeat)
	e.POST("/generateTransferZKP", generateTransferZKP)
	e.POST("/generateReclaimZKP", generateReclaimZKP)
	err := e.Start(":8081")
	if err != nil {
		log.Fatal(err)
	}
}

func (p program) Stop(s service.Service) error {
	return nil
}

func main() {
	svcConfig := &service.Config{
		Name:        "GoServiceExampleSimple",
		DisplayName: "Go Service Example",
		Description: "This is an example Go service.",
	}
	s, err := service.New(&program{}, svcConfig)
	if err != nil {
		log.Fatal(err)
	}
	logger, err = s.Logger(nil)
	if err != nil {
		log.Fatal(err)
	}
	err = s.Run()
	if err != nil {
		logger.Error(err)
	}
}

func heartbeat(ctx echo.Context) error {
	return nil
}

func generateTransferZKP(ctx echo.Context) error {
	appVersion := ctx.QueryParam("app_version")
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	input := fmt.Sprintf("%d", len(bytes))
	output := C.hello(C.CString(input))
	resp := C.GoString(output)
	C.free(unsafe.Pointer(output))
	return ctx.JSON(http.StatusOK, map[string]interface{}{
		"transfer_zkp":   resp,
		"daemon_version": DaemonVersion,
		"app_version":    appVersion,
	})
}

func generateReclaimZKP(ctx echo.Context) error {
	appVersion := ctx.QueryParam("app_version")
	body := ctx.Request().Body
	defer body.Close()
	bytes, err := ioutil.ReadAll(body)
	if err != nil {
		log.Fatal(err)
	}
	input := fmt.Sprintf("%d", len(bytes))
	output := C.hello(C.CString(input))
	resp := C.GoString(output)
	C.free(unsafe.Pointer(output))
	return ctx.JSON(http.StatusOK, map[string]interface{}{
		"reclaim_zkp":    resp,
		"daemon_version": DaemonVersion,
		"app_version":    appVersion,
	})
}
