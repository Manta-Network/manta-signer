package main

/*
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"log"
	"unsafe"

	"github.com/Manta-Network/Manta-Singer/utils"
	"github.com/pkg/errors"
	"github.com/wailsapp/wails/v2"
)

type Service struct {
	runtime *wails.Runtime
	rootSeed    *[64]byte
	userIsSignedIn *bool
}

func NewService(rootSeed *[64]byte, userIsSignedIn *bool) *Service {
	return &Service{
		rootSeed: rootSeed,
		userIsSignedIn: userIsSignedIn,
	}
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

func (c *Service) CreateAccount(password string) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	res := C.create_account(C.CString(password), &outBufferRef, &outLen)
	if (res == 0) {
		recovery_phrase := C.GoString(outBufferRef)
		log.Print("recovery_phrase", recovery_phrase)
		C.free(unsafe.Pointer(outBufferRef))
		return C.GoString(outBufferRef)
	}
	log.Print("error creating account")
	return "";
}

func (c *Service) LoadRootSeed(password string) bool {
	var outBuffer []byte
	outBufferRef := C.CBytes(outBuffer)
	res := C.load_root_seed(C.CString(password), &outBufferRef)
	if (res == 0) {
		rootSeed := C.GoBytes(outBufferRef, C.int(64))
		var rootSeedSized [64]byte
		copy(rootSeedSized[:], rootSeed)
		*c.rootSeed = rootSeedSized
		*c.userIsSignedIn = true
		// println("rootSeed in service", c.rootSeed)
		C.free(outBufferRef)
		return true
	}
	return false
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
		return errors.New("incorrect password")
	}
}
