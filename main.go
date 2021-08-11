package main

import (
	"log"
	"os"

	"github.com/kardianos/service"
	"github.com/labstack/echo/v4"
	"github.com/urfave/cli/v2"
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
	e.POST("/deriveShieldedAddress", deriveShieldedAddress)
	e.POST("/generateAsset", generateAsset)
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
