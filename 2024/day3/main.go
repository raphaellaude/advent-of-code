package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
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
		muls := FindMuls(line)

		for _, mul := range muls {
			a, b := ParseMul(mul)
			total += a * b
		}
	}

	return total
}

func FindMuls(str string) []string {
	match := regexp.MustCompile(`mul\([0-9]+,[0-9]+\)`)
	return match.FindAllString(str, -1)
}

func ParseMul(str string) (int, int) {
	match := regexp.MustCompile(`[0-9]+`)
	matches := match.FindAllString(str, -1)

	a, err := strconv.Atoi(matches[0])
	check(err)

	b, err := strconv.Atoi(matches[1])
	check(err)

	return a, b
}
