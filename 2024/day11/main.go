package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input_file := "./input.txt"
	result1 := Part1(input_file)
	fmt.Println("Part 1 result: ", result1)

	result2 := Part2(input_file)
	fmt.Println("Part 2 result: ", result2)
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

	cache := make(map[CacheVal]int)

	for scanner.Scan() {
		line := scanner.Text()
		vals := strings.Split(line, " ")

		for _, val := range vals {
			total += CountStones(val, 25, &cache)
		}
	}

	return total
}

func Part2(input_file string) int {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	total := 0

	cache := make(map[CacheVal]int)

	for scanner.Scan() {
		line := scanner.Text()
		vals := strings.Split(line, " ")

		for _, val := range vals {
			total += CountStones(val, 75, &cache)
		}
	}

	fmt.Println("Cache size: ", len(cache))

	return total
}

type CacheVal struct {
	stone string
	steps int
}

func CountStones(stone string, steps int, cache *map[CacheVal]int) int {
	if val, ok := (*cache)[CacheVal{stone, steps}]; ok {
		return val
	}

	if steps == 0 {
		return 1
	}

	if stone == "0" {
		return CountStones("1", steps-1, cache)
	}

	if len(stone)%2 == 0 {
		a, b := Split(stone)
		a_tot := CountStones(a, steps-1, cache)
		(*cache)[CacheVal{a, steps - 1}] = a_tot

		b_tot := CountStones(b, steps-1, cache)
		(*cache)[CacheVal{b, steps - 1}] = b_tot

		return a_tot + b_tot
	}

	vint, err := strconv.Atoi(stone)
	check(err)
	vint *= 2024
	stone = strconv.Itoa(vint)
	t := CountStones(stone, steps-1, cache)
	(*cache)[CacheVal{stone, steps - 1}] = t

	return t
}

func Split(stone string) (string, string) {
	l := len(stone)
	a := stone[:l/2]
	b, err := strconv.Atoi(stone[l/2:])
	check(err)

	return a, strconv.Itoa(b)
}
