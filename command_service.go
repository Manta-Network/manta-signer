package main

import (
	"github.com/wailsapp/wails/v2"
)

type CommandService struct {
	runtime *wails.Runtime
}

func NewCommandService() *CommandService {
	return &CommandService{}
}

func (c *CommandService) WindowHide() {
	c.runtime.Window.Hide()
}
