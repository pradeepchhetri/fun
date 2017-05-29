package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func simpleSieve(limit int, prime []int) {
	var mark = make([]bool, limit+1) // Array of booleans

	// Initialize as prime
	for l := range mark {
		mark[l] = true
	}

	// Make 0 and 1 as non-prime
	mark[0] = false
	mark[1] = false

	// Unmark non-prime numbers
	for p := 2; p*p < limit; p++ {
		if mark[p] == true {
			for j := p * 2; j <= limit; j = j + p {
				mark[j] = false
			}
		}
	}

	// Print all prime numbers
	for k := range mark {
		if mark[k] {
			prime = append(prime, k)
			fmt.Println(k)
		}
	}
}

func segmentedSieve(n int) {
	limit := math.Floor(math.Sqrt(n))
	prime := make([]bool, n+1)

	simpleSieve(limit, prime)

	var low = limit
	var high = 2 * limit

	for low < n {
		var mark = make([]bool, limit+1)

		// Initialize as prime
		for l := range mark {
			mark[l] = true
		}

	}
}

func main() {
	var i, cases int64 // Num of test cases

	reader := bufio.NewReader(os.Stdin)
	text, err := reader.ReadString('\n')
	if err != nil {
		fmt.Println(err)
	}
	text = strings.TrimSpace(text)

	cases, err = strconv.ParseInt(text, 10, 64)
	if err != nil {
		fmt.Println(err)
	}

	for i = 1; i <= cases; i++ {
		var min int64 // Minimum prime number
		var max int64 // Maximum prime number
		text, err = reader.ReadString('\n')
		if err != nil {
			fmt.Println(err)
		}
		words := strings.Fields(text)

		min, _ = strconv.ParseInt(words[0], 10, 64)
		max, _ = strconv.ParseInt(words[1], 10, 64)

	}
}
