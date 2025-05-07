/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/flevin58/pngtool/png"
	"github.com/spf13/cobra"
)

// dumpCmd represents the dump command
var dumpCmd = &cobra.Command{
	Use:     "dump",
	Short:   "Dumps the png tags to standard output",
	Long:    `Dumps the png tags to standard output.`,
	Example: "dump myimage.png",
	Args:    cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		path := args[0]
		fmt.Println("Dumping PNG file:", path)
		png, err := png.New(path)
		if err != nil {
			cobra.CompErrorln(err.Error())
			return
		}
		png.Dump()
	},
}

func init() {
	rootCmd.AddCommand(dumpCmd)
}
