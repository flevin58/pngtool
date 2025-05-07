/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/flevin58/pngtool/png"
	"github.com/spf13/cobra"
)

// injectCmd represents the inject command
var injectCmd = &cobra.Command{
	Use:   "inject",
	Short: "Injects a text message into the given png image",
	Long:  `Injects e text message into the given png image`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		path := args[0]
		fmt.Printf("Injecting message to PNG file: %s\n", path)
		png, err := png.New(path)
		if err != nil {
			cobra.CompErrorln(err.Error())
			return
		}
		output, _ := cmd.Flags().GetString("output")
		message, _ := cmd.Flags().GetString("message")
		if err != nil {
			return
		}
		png.Inject(output, message)
	},
}

func init() {
	rootCmd.AddCommand(injectCmd)
	injectCmd.Flags().StringP("message", "m", "", "The message to embed in the png image")
	injectCmd.Flags().StringP("output", "o", "", "The output png image with the embedded message")
}
