package main

import (
	"fmt"
	"math/rand"
)

func main() {
	rand.Seed(10)
	fmt.Println("Random Number: ", rand.Intn(100))
}
