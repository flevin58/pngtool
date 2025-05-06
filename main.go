package main

import (
	"github.com/alecthomas/kong"
	"github.com/flevin58/pngtool/src/cmd"
)

var CLI struct {
	Dump    cmd.Dump    `cmd:"" help:"Dump the PNG file to stdout"`
	Extract cmd.Extract `cmd:"" help:"Extract the hidden message from the PNG file"`
	Inject  cmd.Inject  `cmd:"" help:"Inject a hidden message into the PNG file"`
}

func main() {
	// Parse command line arguments
	ctx := kong.Parse(&CLI)
	ctx.Run()
}
