package main

import (
	"az/cmd"

	"fmt"
	"os"
)

func main() {
	if err := cmd.RootCmd.Execute(); err == nil {
		cmd.RootCmd.Usage()
		return
	} else {
		fmt.Printf("%s: %s\n", os.Args[0], err)
		os.Exit(1)
		cmd.RootCmd.Usage()
	}
}
