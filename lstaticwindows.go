// +build windows

package main

/*
#cgo LDFLAGS: -L./lib/windows -lzkp -ladvapi32 -lws2_32 -luserenv
*/
import "C"
