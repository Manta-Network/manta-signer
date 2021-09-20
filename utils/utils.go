package utils

import (
	"os"
)

func AccountCreated() bool {
	if _, err := os.Stat("root_seed.aes"); err == nil {
		return true
	} else if os.IsNotExist(err) {
		return false
	} else {
		return false
	}
}
