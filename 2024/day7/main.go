package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
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

		bleh := strings.Split(line, ": ")
		val, err := strconv.Atoi(bleh[0])
		check(err)

		s_nums := strings.Split(bleh[1], " ")

		var nums []int

		for _, num := range s_nums {
			lval, err := strconv.Atoi(num)
			check(err)
			nums = append(nums, lval)
		}

		if CountValidEquation(val, nums) > 0 {
			total += val
		}

	}

	return total
}

func CountValidEquation(val int, nums []int) int {
	if len(nums) == 2 {
		var tot int
		a, b := nums[0], nums[1]
		if a*b == val {
			tot++
		}
		if a+b == val {
			tot++
		}
		return tot
	} else {
		a, b := nums[0], nums[1]
		return CountValidEquation(val, slices.Concat([]int{a * b}, nums[2:])) + CountValidEquation(val, slices.Concat([]int{a + b}, nums[2:]))
	}
}
