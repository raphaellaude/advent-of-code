package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	// Part 1
	input_file := "./input.txt"
	result := Part1(input_file)
	fmt.Println("Part 1 result: ", result)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func Part1(input_file string) int {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	total := 0
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
	}

	return total
}
