package main

import (
	"fmt"
	"github.com/labstack/echo/v4"
	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/menu"
	"github.com/wailsapp/wails/v2/pkg/menu/keys"
	"github.com/wailsapp/wails/v2/pkg/options/dialog"
	"log"
	"sync"
	"time"
)

const (
	ConcurrentIncomeURL = 1
)

// app struct
type app struct {
	addr    string
	svr     *echo.Echo
	runtime *wails.Runtime

	// appMenu 不可视，主要用途热键
	appMenu *menu.Menu
	// 托盘菜单
	defaultTrayMenu *menu.TrayMenu
	//startsAtLoginMenu *menu.MenuItem
	autoUpdateMenu *menu.MenuItem
	appUpdatesMenu *menu.MenuItem

	CommandService *CommandService
	lock           sync.Mutex
	// 是否输出调试信息
	Verbose bool
	// 是否暗黑模式 todo 估计windows不支持
	isDarkMode bool

	defaultTrayMenuActive bool

	// incomingURLSemaphore 用通道确保一次只处理一个url事件
	incomingURLSemaphore chan struct{}
}

// newApp creates a new app application struct
func newApp(addr string) (*app, error) {
	app := &app{
		addr:                 addr,
		incomingURLSemaphore: make(chan struct{}, ConcurrentIncomeURL),
	}
	app.appMenu = menu.NewMenuFromItems(
		menu.AppMenu(),
		menu.EditMenu(),
		menu.WindowMenu(),
	)

	// 初始化要导出的服务
	app.CommandService = NewCommandService()

	// 自动更新
	app.appUpdatesMenu = &menu.MenuItem{
		Type:  menu.TextType,
		Label: "Check for updates...",
	}

	// 启动时自动更新
	app.autoUpdateMenu = &menu.MenuItem{
		Type:  menu.CheckboxType,
		Label: "Update automatically",
	}

	// 是否启动运行
	//app.startsAtLoginMenu = &menu.MenuItem{
	//	Type:    menu.CheckboxType,
	//	Label:   "Start at Login",
	//	Checked: false,
	//}
	//startsAtLogin, err := mac.StartsAtLogin()
	//if err != nil {
	//	if app.Verbose {
	//		log.Println("start at login:", err)
	//	}
	//	app.startsAtLoginMenu.Label = "Start at Login"
	//	app.startsAtLoginMenu.Checked = true
	//} else {
	//	app.startsAtLoginMenu.Checked = startsAtLogin
	//}

	app.defaultTrayMenu = &menu.TrayMenu{
		Label: DaemonName,
		Menu:  app.newTrayMenu(),
	}

	return app, nil
}

func (b *app) startupServer() {
	var heartbeat = func(ctx echo.Context) error {
		return nil
	}
	e := echo.New()
	e.GET("/heartbeat", heartbeat)
	e.POST("/generateTransferZKP", generateTransferZKP)
	e.POST("/generateReclaimZKP", generateReclaimZKP)
	e.POST("/deriveShieldedAddress", deriveShieldedAddress)
	e.POST("/generateAsset", generateAsset)
	err := e.Start(b.addr)
	if err != nil {
		log.Fatal(err)
	}
}

// startup is called at application startup
func (b *app) startup(runtime *wails.Runtime) {
	b.setDarkMode(runtime.System.IsDarkMode())
	// 暗黑模式更新
	runtime.Events.OnThemeChange(func(darkMode bool) {
		// keep track of dark mode changing, and refresh all
		// plugins if it does.
		b.setDarkMode(darkMode)
		b.refreshAll()
	})
	b.runtime = runtime
	b.CommandService.runtime = runtime
	b.refreshAll()

	// 暴露对外服务接口
	go b.startupServer()
	// 准备自动更新
	go func() {
		// 等待检查更新
		time.Sleep(10 * time.Second)
		for {
			// todo app.checkForUpdates(true)
			// 12小时等待再次检查更新
			time.Sleep(12 * time.Hour)
		}
	}()
}

func (b *app) refreshAll() {
	b.lock.Lock()
	defer b.lock.Unlock()
	if b.defaultTrayMenuActive {
		// only default menu - remove it
		b.runtime.Menu.DeleteTrayMenu(b.defaultTrayMenu)
		b.defaultTrayMenuActive = false
	}
	b.runtime.Menu.SetTrayMenu(b.defaultTrayMenu)
	b.defaultTrayMenuActive = true
	return
}

// shutdown is called at application termination
func (b *app) shutdown() {
	// Perform your teardown here
}

func (b *app) newTrayMenu() *menu.Menu {
	var items []*menu.MenuItem
	items = append(items, &menu.MenuItem{
		Type:     menu.TextType,
		Label:    fmt.Sprintf("manta-singer (%s)", version),
		Disabled: true,
	})
	items = append(items, &menu.MenuItem{
		Type:  menu.TextType,
		Label: "Open signer",
		Click: func(_ *menu.CallbackData) {
			b.runtime.Window.Show()
		},
	})
	items = append(items, b.appUpdatesMenu)
	items = append(items, b.autoUpdateMenu)
	//items = append(items, b.startsAtLoginMenu)
	items = append(items, menu.Separator())
	items = append(items, &menu.MenuItem{
		Type:        menu.TextType,
		Label:       "Quit",
		Accelerator: keys.CmdOrCtrl("q"),
		Click:       b.onQuitMenuClicked,
	})
	m := &menu.Menu{Items: items}
	return m
}

func (b *app) onQuitMenuClicked(_ *menu.CallbackData) {
	b.runtime.Quit()
}

func (b *app) setDarkMode(darkMode bool) {
	b.lock.Lock()
	defer b.lock.Unlock()
	b.isDarkMode = darkMode
}

func (b *app) handleIncomeURL(url string) {
	b.incomingURLSemaphore <- struct{}{}
	defer func() {
		<-b.incomingURLSemaphore
	}()
	log.Println("incoming URL: handleIncomingURL", url)
	incomingURL, err := parseIncomingURL(url)
	if err != nil {
		_, err2 := b.runtime.Dialog.Message(&dialog.MessageDialog{
			Type:         dialog.ErrorDialog,
			Title:        "Invalid URL",
			Message:      err.Error(),
			Buttons:      []string{"OK"},
			CancelButton: "OK",
		})
		if err2 != nil {
			log.Println(err2)
			return
		}
		return
	}
	switch incomingURL.Action {
	case "show":
		b.runtime.Window.Show()
	default:
		log.Printf("incoming URL: skipping, unknown action %q\n", incomingURL.Action)
	}
}
