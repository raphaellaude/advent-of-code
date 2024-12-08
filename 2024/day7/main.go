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
	result1, result2 := Main(input_file)
	fmt.Println("Part 1 result: ", result1)
	fmt.Println("Part 2 result: ", result2)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func Main(input_file string) (int, int) {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	var part1 int
	var part2 int

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
			part1 += val
			part2 += val
		} else {
			if CountValidEquationWithConcat(val, nums) {
				part2 += val
			}
		}
	}

	return part1, part2
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

func ConcatInts(a int, b int) int {
	s := strconv.Itoa(a) + strconv.Itoa(b)
	v, err := strconv.Atoi(s)
	check(err)
	return v
}

func CountValidEquationWithConcat(val int, nums []int) bool {
	if len(nums) == 2 {
		a, b := nums[0], nums[1]
		return ConcatInts(a, b) == val || a*b == val || a+b == val
	} else {
		a, b := nums[0], nums[1]
		return CountValidEquationWithConcat(val, slices.Concat([]int{ConcatInts(a, b)}, nums[2:])) || CountValidEquationWithConcat(val, slices.Concat([]int{a * b}, nums[2:])) || CountValidEquationWithConcat(val, slices.Concat([]int{a + b}, nums[2:]))
	}
}
