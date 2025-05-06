package cmd

import (
	"fmt"

	"github.com/flevin58/pngtool/src/png"
)

type Dump struct {
	Path string `arg:"" name:"path" help:"Path to the PNG file to dump" type:"existingfile"`
}

func (d *Dump) Run() error {
	fmt.Println("Dumping PNG file:", d.Path)
	png, err := png.New(d.Path)
	if err != nil {
		return fmt.Errorf("failed to open file: %w", err)
	}
	png.Dump()
	return nil
}
