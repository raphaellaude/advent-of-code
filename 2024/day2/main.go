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

	// result := CheckReports(input_file, 0)
	// fmt.Println("Part 1 result: ", result)

	result2 := CheckReports(input_file, 1)
	fmt.Println("Part 2 result: ", result2)
}

func CheckReports(input_file string, tolerance int) int {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	total := 0
	for scanner.Scan() {
		line := scanner.Text()
		nums := strings.Split(line, " ")

		fmt.Println("Checking: ", nums)
		if IsValid(nums, tolerance) {
			fmt.Println("Valid path: ", nums)
			total++
		} else {
			fmt.Println("Invalid path")
		}
	}

	return total
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func Abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func IsValid(nums []string, tolerance int) bool {
	var direction int
	var num_errors int

	for i := 0; i < len(nums)-1; i++ {
		a, err := strconv.Atoi(nums[i])
		check(err)
		b, err := strconv.Atoi(nums[i+1])
		check(err)

		new_step := b - a

		if i == 0 && new_step != 0 {
			direction = new_step / Abs(new_step)
		}

		if new_step == 0 || Abs(new_step) > 3 || (direction == -1 && new_step > 0) || (direction == 1 && new_step < 0) {
			num_errors++

			if num_errors > tolerance {
				return false
			}

			nums2 := slices.Clone(nums)

			nums1 := slices.Delete(nums, i, i+1)
			nums2 = slices.Delete(nums2, i+1, i+2)

			to_test := []bool{IsValid(nums1, tolerance-1), IsValid(nums2, tolerance-1)}
			return Any(to_test, func(t bool) bool { return t })
		}
	}

	return true
}

func Any[T any](ts []T, pred func(T) bool) bool {
	for _, t := range ts {
		if pred(t) {
			return true
		}
	}
	return false
}
