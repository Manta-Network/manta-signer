package main

/*
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"github.com/Manta-Network/Manta-Singer/utils"
	"github.com/pkg/errors"
	"github.com/wailsapp/wails/v2"
	dialogoptions "github.com/wailsapp/wails/v2/pkg/options/dialog"
	"os"
)

type Service struct {
	runtime *wails.Runtime
}

func NewService() *Service {
	return &Service{}
}

func (c *Service) WindowHide() {
	c.runtime.Window.Hide()
}

func (c *Service) WindowShow() {
	c.runtime.Window.Show()
}

func (c *Service) AccountCreated() bool {
	return utils.AccountCreated()
}

// AcquireSeedByPassword 通过密码获取助记词
func (c *Service) AcquireSeedByPassword(password string) (string, error) {
	err := utils.CreateAccountCreatedFlag()
	if err != nil {
		return "", err
	}
	return "hello world", nil
}

func (c *Service) RecoverAccount(seed, password string) error {
	return nil
}

func (c *Service) Unlock(password string) error {
	if password == "12345678" {
		return nil
	} else {
		return errors.New("密码不正确")
	}
}

func (c *Service) SaveCSV(seed string) error {
	path, err := c.runtime.Dialog.SaveFile(&dialogoptions.SaveDialog{
		DefaultFilename: "phrase.csv",
	})
	if err != nil {
		return err
	}
	f, err := os.Create(path) //传递文件路径
	if err != nil {           //有错误
		return err
	}

	//使用完毕，需要关闭文件
	defer f.Close()

	_, err = f.WriteString(seed)
	if err != nil {
		return err
	}
	return nil
}
