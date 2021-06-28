package main

/*
#cgo LDFLAGS: -L./lib -lzkp
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"github.com/kardianos/service"
	"github.com/labstack/echo/v4"
	"github.com/urfave/cli/v2"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"unsafe"
)

const (
	DaemonName        = "manta-daemon"
	DaemonDisplayName = "manta-daemon"
	DaemonUsage       = "This is an a daemon service for manta."
	DaemonVersion     = "0.1.0"
)

var (
	addr string
)

var logger service.Logger

type program struct{}

func (p program) Start(s service.Service) error {
	go p.run()
	return nil
}

func (p program) run() {
	e := echo.New()
	e.GET("/heartbeat", heartbeat)
	e.POST("/generateTransferZKP", generateTransferZKP)
	e.POST("/generateReclaimZKP", generateReclaimZKP)
	err := e.Start(addr)
	if err != nil {
		log.Fatal(err)
	}
}

func (p program) Stop(s service.Service) error {
	return nil
}

func main() {
	app := cli.NewApp()
	app.Name = DaemonName
	app.Version = DaemonVersion
	app.Flags = []cli.Flag{
		&cli.StringFlag{
			Name:        "addr",
			Value:       ":29986",
			Usage:       "set the http addr for daemon progress to listen",
			Destination: &addr,
		},
	}
	app.Action = func(context *cli.Context) error {
		svcConfig := &service.Config{
			Name:        DaemonName,
			DisplayName: DaemonDisplayName,
			Description: DaemonUsage,
		}
		s, err := service.New(&program{}, svcConfig)
		if err != nil {
			log.Fatal(err)
		}
		logger, err = s.Logger(nil)
		if err != nil {
			log.Fatal(err)
		}
		return s.Run()
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
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
