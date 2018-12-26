package cmd

import (
	"fmt"
	"os"
	"github.com/spf13/cobra"
)

func rootCmd() *cobra.Command {
	cmd := &cobra.Command {
		Use:   "az",
		Short: "get az",
		Long:  "get az",
		RunE: az,
	}
	cmd.Flags().BoolP("debug", "d", false, "out encoding")
	cmd.Flags().IntP("max", "m", 13772, "max entries")
	cmd.Flags().StringP("code", "c", "utf-8", "out encoding")
	return cmd
}

func az(cmd *cobra.Command, args []string) error {
	fmt.Printf("QQQ az:\n")
	return nil
}

func Execute() {
	if err := rootCmd().Execute(); err == nil {
		return
	} else {
		fmt.Printf("%s: %s\n", os.Args[0], err)
		os.Exit(1)
	}
}
