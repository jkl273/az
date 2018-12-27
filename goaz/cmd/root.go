package cmd

import (
	"fmt"
	"math/rand"
	"time"
	"reflect"
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
	cmd.AddCommand(testCmd())
	return cmd
}

func az(cmd *cobra.Command, args []string) error {
	max, _ := cmd.Flags().GetInt("max")
	debug, _ := cmd.Flags().GetBool("debug")
	code, _ := cmd.Flags().GetString("code")
	fmt.Printf("type: %s\n", reflect.TypeOf(max))
	fmt.Printf("max: %d\n", max)
	fmt.Printf("debug: %t\n", debug)
	fmt.Printf("code: %s\n", code)

	rand.Seed(time.Now().UnixNano())
	fmt.Println(rand.Intn(max)) // [0, max)
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
