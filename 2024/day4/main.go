package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	input_file := "./input.txt"
	result := Part1(input_file)
	fmt.Println("Part 1 result: ", result)

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

	graph, line_length, max_index := BuildGraph(scanner)

	for i := 0; i <= max_index; i++ {
		xmases := CountXmases(graph, i, line_length, max_index)
		total += xmases
	}

	return total
}

func CountXmases(graph map[int]string, i int, line_length int, max_index int) int {
	var xmases int
	c := graph[i]

	a, b := i/line_length, i%line_length

	if c != "X" && c != "S" {
		return xmases
	}

	// Up
	if a >= 3 {
		xmases += CheckForXMAS(c, []int{i - line_length, i - 2*line_length, i - 3*line_length}, graph)
	}

	// Left
	if b >= 3 {
		xmases += CheckForXMAS(c, []int{i - 1, i - 2, i - 3}, graph)
	}

	// Diagonal up and to the left
	if a >= 3 && b >= 3 {
		xmases += CheckForXMAS(c, []int{i - line_length - 1, i - 2*line_length - 2, i - 3*line_length - 3}, graph)
	}

	// Diagonal up and to the right
	if a >= 3 && b <= line_length-4 {
		xmases += CheckForXMAS(c, []int{i - line_length + 1, i - 2*line_length + 2, i - 3*line_length + 3}, graph)
	}

	return xmases
}

func CheckForXMAS(s string, idxs []int, graph map[int]string) int {
	for _, idx := range idxs {
		s += graph[idx]
	}

	if s == "XMAS" || s == "SAMX" {
		return 1
	}

	return 0
}

func BuildGraph(scanner *bufio.Scanner) (map[int]string, int, int) {
	graph := make(map[int]string)

	i := 0
	j := 0
	var line_length int
	var node_idx int

	for scanner.Scan() {
		line := scanner.Text()
		line_length = len(line)

		for _, c := range line {
			node_idx = i*line_length + j
			graph[node_idx] = string(c)
			j++
		}

		i++
		j = 0
	}
	return graph, line_length, node_idx
}

func Part2(input_file string) int {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	total := 0

	graph, line_length, max_index := BuildGraph(scanner)

	for i := 0; i <= max_index; i++ {
		xmases := CountMases(graph, i, line_length, max_index)
		total += xmases
	}

	return total
}

func CountMases(graph map[int]string, i int, line_length int, max_index int) int {
	var mases int
	c := graph[i]

	a, b := i/line_length, i%line_length

	if c != "M" && c != "S" {
		return mases
	}

	// Diagonal up and to the left
	if a >= 2 && b >= 2 {
		mases += CheckForMAS(c, []int{i - line_length - 1, i - 2*line_length - 2}, graph)
	}

	// Three back and up and to the right
	if a >= 2 && b >= 2 {
		mases += CheckForMAS("", []int{i - 2, i - line_length - 1, i - 2*line_length}, graph)
	}

	if mases == 2 {
		return 1
	}

	return 0
}

func CheckForMAS(s string, idxs []int, graph map[int]string) int {
	for _, idx := range idxs {
		s += graph[idx]
	}

	if s == "MAS" || s == "SAM" {
		return 1
	}

	return 0
}
