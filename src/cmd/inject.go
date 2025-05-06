package cmd

import (
	"fmt"

	"github.com/flevin58/pngtool/src/png"
)

type Inject struct {
	Path    string `arg:"" name:"path" help:"Path to the source PNG file" type:"existingfile"`
	Output  string `short:"o" help:"Path to the new PNG file" type:"path"`
	Message string `short:"m" required:"" help:"The message to embed"`
}

func (i *Inject) Run() error {
	fmt.Printf("Injecting message to PNG file: %s\n", i.Path)
	png, err := png.New(i.Path)
	if err != nil {
		return fmt.Errorf("failed to open file: %w", err)
	}
	png.Inject(i.Output, i.Message)
	return nil
}
