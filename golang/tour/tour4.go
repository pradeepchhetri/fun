package main

import "fmt"

func swap(a string, b string) (string, string) {
	return b, a
}

func swap(a int, b int) (int, int) {
	return b, a
}

func main() {
	fmt.Println(swap("Hello", "World"))
}
