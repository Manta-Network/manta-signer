package main

/*
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"github.com/pkg/errors"
	"github.com/wailsapp/wails/v2"
	"unsafe"
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
	seed := C.generate_recovery_phrase(C.CString(password))
	defer C.free(unsafe.Pointer(seed))
	return C.GoString(seed), nil
}

func (c *Service) RestoreVaultBySeed(seed, password string) error {
	c.logged = true
	res := C.modify_password_by_recovery_phrase(C.CString(seed), C.CString(password))
	if res == 0 {
		return nil
	}
	return errors.New("不正确的助记词")
}

func (c *Service) Unlock(password string) error {
	if C.verify_password(C.CString(password)) == 0 {
		return nil
	} else {
		return errors.New("密码不正确")
	}
}
