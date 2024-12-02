package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	// Part 1
	input_file := "./input.txt"
	result := Part1(input_file)
	fmt.Printf("Part 1 result go %d", result)
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

	var total float64

	var l1 []float64
	var l2 []float64

	for scanner.Scan() {
		line := scanner.Text()
		split := strings.Split(line, "   ")
		_a, _b := split[0], split[1]

		a, err := strconv.ParseFloat(_a, 64)
		check(err)

		b, err := strconv.ParseFloat(_b, 64)
		check(err)

		l1 = append(l1, a)
		l2 = append(l2, b)
	}

	sort.Float64s(l1)
	sort.Float64s(l2)

	for idx := 0; idx < len(l1); idx++ {
		total += math.Abs(l1[idx] - l2[idx])
	}

	return int(total)
}
