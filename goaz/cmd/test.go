package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
)

func testCmd() *cobra.Command {
	cmd := &cobra.Command {
		Use:   "test",
		Short: "test short",
		Long:  "test long",
		RunE: test,
	}
	cmd.Flags().BoolP("testb", "b", false, "bool option")
	cmd.Flags().IntP("testi", "i", 273, "int option")
	cmd.Flags().StringP("tests", "s", "test", "string option")
	return cmd
}

func test(cmd *cobra.Command, args []string) error {
	fmt.Printf("QQQ test:\n")
	return nil
}

