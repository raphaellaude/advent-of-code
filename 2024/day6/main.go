package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
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

type MappedArea struct {
	grouds     string
	rows       int
	row_length int
}

func (mapped_area *MappedArea) ViewVisitedLocations(locations map[int]bool) string {
	var new_grounds string

	for idx, val := range mapped_area.grouds {
		if idx%mapped_area.row_length == 0 && idx != 0 {
			new_grounds += "\n"
		}

		if val == '^' {
			new_grounds += string(val)
			continue
		}

		if _, ok := locations[idx]; ok {
			new_grounds += "X"
			continue
		}

		new_grounds += string(val)
	}

	return new_grounds
}

type ExititedMappedAreaError struct {
	direction Direction
}

func (e *ExititedMappedAreaError) Error() string {
	return fmt.Sprintf("Guard exitited mapped area going %d", e.direction)
}

type Direction int

const (
	up Direction = iota
	right
	down
	left
)

type Guard struct {
	loc       int
	direction Direction
}

func (guard *Guard) GetNextPosition(mapped_area MappedArea) (int, error) {
	if guard.direction == up && guard.loc < mapped_area.row_length {
		return -1, &ExititedMappedAreaError{direction: up}
	}

	if guard.direction == right && guard.loc%mapped_area.row_length == mapped_area.row_length-1 {
		return -1, &ExititedMappedAreaError{direction: right}
	}

	if guard.direction == left && guard.loc%mapped_area.row_length == 0 {
		return -1, &ExititedMappedAreaError{direction: left}
	}

	if guard.direction == down && (guard.loc+mapped_area.row_length) >= len(mapped_area.grouds) {
		return -1, &ExititedMappedAreaError{direction: down}
	}

	if guard.direction == up {
		return guard.loc - mapped_area.row_length, nil
	}

	if guard.direction == right {
		return guard.loc + 1, nil
	}

	if guard.direction == left {
		return guard.loc - 1, nil
	}

	if guard.direction == down {
		return guard.loc + mapped_area.row_length, nil
	}

	return -1, &ExititedMappedAreaError{}
}

func (guard *Guard) TurnRight(mapped_area MappedArea) {
	guard.direction = (guard.direction + 1) % 4
}

func Main(input_file string) (int, int) {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	var mapped_area MappedArea

	for scanner.Scan() {
		line := scanner.Text()
		mapped_area.grouds += line
		mapped_area.rows++
	}

	mapped_area.row_length = len(mapped_area.grouds) / mapped_area.rows

	start_pos := strings.Index(mapped_area.grouds, "^")

	if start_pos == -1 {
		return -1, -1
	}

	guard := Guard{loc: start_pos, direction: up}

	locations := make(map[int]bool)
	locations[guard.loc] = true

	for {
		loc, err := guard.GetNextPosition(mapped_area)

		if err != nil {
			break
		}

		if mapped_area.grouds[loc] == '#' {
			guard.TurnRight(mapped_area)
		} else {
			guard.loc = loc
		}

		locations[guard.loc] = true
	}

	var p2 int

	for loc := range locations {
		val := mapped_area.grouds[loc]
		if val != '#' && val != '^' {
			new_mapped_area := MappedArea{replaceAtIndex(mapped_area.grouds, 'O', loc), mapped_area.row_length, mapped_area.rows}

			if CheckForLoop(start_pos, up, new_mapped_area) {
				p2++
			}
		}
	}

	fo, err := os.Create("output.txt")
	check(err)

	visited_grounds := mapped_area.ViewVisitedLocations(locations)
	fo.Write([]byte(visited_grounds))

	return len(locations), p2
}

func replaceAtIndex(in string, r rune, i int) string {
	out := []rune(in)
	out[i] = r
	return string(out)
}

var bleh int

func CheckForLoop(starting_pos int, starting_dir Direction, mapped_area MappedArea) bool {
	guard := Guard{loc: starting_pos, direction: starting_dir}
	visits := make(map[Guard]bool)

	for {
		if _, ok := visits[guard]; ok {
			return true
		} else {
			visits[guard] = true
		}

		loc, err := guard.GetNextPosition(mapped_area)

		if err != nil {
			return false
		}

		if slices.Contains([]byte{'#', 'O'}, mapped_area.grouds[loc]) {
			guard.TurnRight(mapped_area)
		} else {
			guard.loc = loc
		}
	}
}
