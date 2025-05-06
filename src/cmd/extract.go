package cmd

import (
	"fmt"

	"github.com/flevin58/pngtool/src/png"
)

type Extract struct {
	Path string `arg:"" name:"path" help:"Path to the PNG file where the hidden data is stored" type:"existingfile"`
}

func (e *Extract) Run() error {
	fmt.Println("Extracting message from PNG file:", e.Path)
	png, err := png.New(e.Path)
	if err != nil {
		return fmt.Errorf("failed to open file: %w", err)
	}
	message := png.SecretMessage()
	if len(message) > 0 {
		fmt.Println("Secret message: ", message)
	} else {
		fmt.Println("No secret message found!")
	}
	return nil
}
