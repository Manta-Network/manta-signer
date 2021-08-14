package main

import (
	"github.com/pkg/errors"
	"github.com/wailsapp/wails/v2"
)

type Service struct {
	// 是否登录
	logged bool
	// 是否已创建账户
	accountCreated bool
	runtime        *wails.Runtime
}

func NewService() *Service {
	return &Service{}
}

func (c *Service) WindowHide() {
	c.runtime.Window.Hide()
}

func (c *Service) LoggedIn() (bool, error) {
	return c.logged, nil
}

func (c *Service) AccountCreated() (bool, error) {
	return c.accountCreated, nil
}

// AcquireSeedByPassword 通过密码获取助记词
func (c *Service) AcquireSeedByPassword(password string) (string, error) {
	// todo 是否需要校验密码
	c.accountCreated = true
	c.logged = true
	return "wealth enrich manual process trap issue olympic stand gravity luggage tissue soon", nil
}

func (c *Service) RestoreVaultBySeed(seed, password string) error {
	c.logged = true
	return nil
}

func (c *Service) Unlock(password string) error {
	if password == "12345678" {
		return nil
	} else {
		return errors.New("密码不正确")
	}
}
