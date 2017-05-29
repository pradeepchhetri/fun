package main

import "fmt"

func add(i int, j int) int {
	return i + j
}

func sub(i int, j int) int {
	return i - j
}

func mul(i int, j int) int {
	return i * j
}

func div(i int, j int) int {
	return i / j
}

func main() {
	fmt.Println("Addition::", add(1, 1))
	fmt.Println("Multiplication::", mul(1, 1))
	fmt.Println("Subtraction::", sub(1, 1))
	fmt.Println("Division::", div(1, 1))
}
