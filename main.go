package main

import (
	_ "embed"
	"fmt"
	"os"
	"runtime"

	"github.com/pkg/errors"
	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/logger"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/mac"
	"github.com/wailsapp/wails/v2/pkg/options/windows"
)

const (
	DaemonName = "manta-signer"
)

//go:embed .version
var version string

func main() {
	println(DaemonName, version)
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "%s\n", err)
		os.Exit(1)
	}
	println(DaemonName + " exited")
}

func run() error {
	app, err := newApp(":29986")
	if err != nil {
		return errors.Wrap(err, "newApp")
	}
	wailsLogLevel := logger.ERROR
	app.Verbose = true
	if app.Verbose {
		wailsLogLevel = logger.DEBUG
	}

	startHidden := true
	if runtime.GOOS == "windows" {
		startHidden = false
	}

	err = wails.Run(&options.App{
		Title:             DaemonName,
		Width:             400,
		Height:            400,
		MinWidth:          400,
		MinHeight:         400,
		StartHidden:       startHidden,
		HideWindowOnClose: true,

		// mac配置
		Mac: &mac.Options{
			WebviewIsTransparent:          true,
			WindowBackgroundIsTranslucent: false,
			TitleBar:                      mac.TitleBarDefault(),
			Menu:                          app.appMenu,
			// 不显示docker图标
			ActivationPolicy: mac.NSApplicationActivationPolicyAccessory,
			URLHandlers: map[string]func(string){
				"manta": app.handleIncomeURL,
			},
		},
		Windows: &windows.Options{
			WebviewIsTransparent:          false,
			WindowBackgroundIsTranslucent: false,
			DisableWindowIcon:             true,
			Menu:                          app.appMenu,
		},

		LogLevel: wailsLogLevel,
		Startup:  app.startup,
		Shutdown: app.shutdown,
		Bind: []interface{}{
			app.Service,
		},
	})
	return err
}
