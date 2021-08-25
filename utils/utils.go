package utils

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

const AccountCreatedFlagName = ".created"

func execDir() string {
	file, _ := exec.LookPath(os.Args[0])
	path, _ := filepath.Abs(file)
	index := strings.LastIndex(path, string(os.PathSeparator))
	ret := path[:index]
	return strings.Replace(ret, "\\", "/", -1)
}

func filename() string {
	return filepath.Join(execDir(), AccountCreatedFlagName)
}

func AccountCreated() bool {
	// 默认相对路径查找
	_, err := os.Stat(filename())
	return !os.IsNotExist(err)
}

func CreateAccountCreatedFlag() error {
	f, err := os.Create(filename())
	defer f.Close()
	return err
}
