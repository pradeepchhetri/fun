package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)
	for {
		text, err := reader.ReadString('\n')
		if text != "42\n" && err == nil {
			fmt.Printf("%s", text)
		} else {
			break
		}
	}
}
