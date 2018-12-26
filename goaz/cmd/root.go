package cmd

import (
	"github.com/spf13/cobra"
)

// RootCmd represents the base command when called without any subcommands
var RootCmd = &cobra.Command{
	Use:   "az",
	Short: "get az",
	Long:  "get az",
	RunE: func(cmd *cobra.Command, args []string) error { return nil },
}

func init() {
	RootCmd.Flags().StringP("code", "c", "utf-8", "out encoding")
}
