/*
Copyright © 2025 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/flevin58/pngtool/png"
	"github.com/spf13/cobra"
)

// extractCmd represents the extract command
var extractCmd = &cobra.Command{
	Use:   "extract",
	Short: "Extracts the embedded message (if present) from the png image.",
	Long:  `Extracts the embedded message (if present) from the png image.`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		path := args[0]
		fmt.Println("Extracting message from PNG file:", path)
		png, err := png.New(path)
		if err != nil {
			cobra.CompErrorln(err.Error())
			return
		}
		message := png.SecretMessage()
		if len(message) > 0 {
			fmt.Println("Secret message: ", message)
		} else {
			fmt.Println("No secret message found!")
		}
	},
}

func init() {
	rootCmd.AddCommand(extractCmd)
}
