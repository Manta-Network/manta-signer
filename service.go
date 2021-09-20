package main

/*
#include <stdlib.h>
#include "./lib/zkp.h"
*/
import "C"
import (
	"unsafe"

	"github.com/Manta-Network/Manta-Singer/utils"
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

// Generates a root seed, and saves it to disk encrypted under the given password
// Returns the root seed represented as a BIP39 recovery phrase
func (c *Service) CreateAccount(password string) string {
	var outBuffer string
	outBufferRef := C.CString(outBuffer)
	var outLen C.size_t
	C.create_account(C.CString(password), &outBufferRef, &outLen)
	recovery_phrase := C.GoString(outBufferRef)
	C.free(unsafe.Pointer(outBufferRef))
	return recovery_phrase
}

// Attempts to load the encrypted root seed from disk into memory with given password
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
		C.free(outBufferRef)
		return true
	}
	return false
}
