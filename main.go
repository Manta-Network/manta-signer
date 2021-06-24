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
			Value:       ":9988",
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
