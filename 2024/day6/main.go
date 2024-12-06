package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"slices"
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

type MappedArea struct {
	grouds     string
	rows       int
	row_length int
}

func (mapped_area *MappedArea) viewVisitedLocations(locations map[int]bool) string {
	var new_grounds string

	for idx, val := range mapped_area.grouds {
		if idx%mapped_area.row_length == 0 && idx != 0 {
			new_grounds += "\n"
		}

		if slices.Contains([]string{"^", ">", "<", "v"}, string(val)) {
			new_grounds += string(val)
			continue
		}

		if _, ok := locations[idx]; ok {
			new_grounds += "X"
		} else {
			new_grounds += string(val)
		}
	}

	return new_grounds
}

type ExititedMappedAreaError struct{}

func (e *ExititedMappedAreaError) Error() string {
	return "Guard exited mapped area"
}

const (
	up = iota
	down
	right
	left
)

type Guard struct {
	loc       int
	direction int
}

func (guard *Guard) GetNextPosition(mapped_area MappedArea) (int, error) {
	if guard.direction == up && guard.loc < mapped_area.row_length {
		fmt.Println("EXITING UP going straight", guard.loc)
		return -1, &ExititedMappedAreaError{}
	}

	if guard.direction == right && guard.loc%mapped_area.row_length == mapped_area.row_length-1 {
		fmt.Println("EXITING RIGHT going straight", guard.loc)
		return -1, &ExititedMappedAreaError{}
	}

	if guard.direction == left && guard.loc%mapped_area.row_length == 0 {
		fmt.Println("EXITING LEFT going straight", guard.loc)
		return -1, &ExititedMappedAreaError{}
	}

	if guard.direction == down && guard.loc >= len(mapped_area.grouds)-mapped_area.row_length {
		fmt.Println("EXITING DOWN going straight", guard.loc)
		return -1, &ExititedMappedAreaError{}
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

func (guard *Guard) TurnRight(mapped_area MappedArea) error {
	// exit down
	if guard.direction == right && guard.loc+mapped_area.row_length >= len(mapped_area.grouds) {
		fmt.Println("EXITING RIGHT turning right", guard.loc)
		return &ExititedMappedAreaError{}
	}

	// exit right
	if guard.direction == up && guard.loc%mapped_area.row_length == mapped_area.row_length {
		fmt.Println("EXITING TOP turning right", guard.loc)
		return &ExititedMappedAreaError{}
	}

	// exit left
	if guard.direction == left && guard.loc < mapped_area.row_length {
		fmt.Println("EXITING LEFT turning right", guard.loc)
		return &ExititedMappedAreaError{}
	}

	// exit xxxx
	if guard.direction == down && guard.loc%mapped_area.row_length == 0 {
		fmt.Println("EXITING BOTTOM turning right", guard.loc)
		return &ExititedMappedAreaError{}
	}

	if guard.direction == up {
		// fmt.Println("turning right going up at guard.loc: ", guard.loc)
		guard.loc += 1
		guard.direction = right
		return nil
	}

	if guard.direction == right {
		// fmt.Println("turning right going down at guard.loc: ", guard.loc)
		guard.loc += mapped_area.row_length
		guard.direction = down
		return nil
	}

	if guard.direction == left {
		// fmt.Println("turning right going up at guard.loc: ", guard.loc)
		guard.loc -= mapped_area.row_length
		guard.direction = up
		return nil
	}

	if guard.direction == down {
		// fmt.Println("turning right going left at guard.loc: ", guard.loc)
		guard.loc -= 1
		guard.direction = left
		return nil
	}

	// This shouldn't happen
	fmt.Println("ERROR TURNING RIGHT")
	return &ExititedMappedAreaError{}
}

func Part1(input_file string) int {
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

	var guard Guard

	carat_re := regexp.MustCompile(`[\^\>v\<]`)
	idx := carat_re.FindStringIndex(mapped_area.grouds)

	if idx != nil {
		a, b := idx[0], idx[1]
		direction := mapped_area.grouds[a:b]

		if direction == "^" {
			guard = Guard{loc: a, direction: up}
		}

		if direction == ">" {
			guard = Guard{loc: a, direction: right}
		}

		if direction == "v" {
			guard = Guard{loc: a, direction: down}
		}

		if direction == "<" {
			guard = Guard{loc: a, direction: left}
		}
	} else {
		return -1
	}

	var locations map[int]bool = make(map[int]bool)
	locations[guard.loc] = true

	for {
		loc, err := guard.GetNextPosition(mapped_area)

		if err != nil {
			break
		}

		next_pos := mapped_area.grouds[loc : loc+1]
		// fmt.Println("next_pos: ", next_pos)

		if next_pos == "#" {
			err := guard.TurnRight(mapped_area)
			if err != nil {
				fmt.Println("Guard exited mapped area at", guard.loc)
				break
			}
		} else {
			guard.loc = loc
		}

		locations[guard.loc] = true
	}

	fo, err := os.Create("output.txt")
	if err != nil {
		panic(err)
	}
	visited_grounds := mapped_area.viewVisitedLocations(locations)
	fo.Write([]byte(visited_grounds))

	return len(locations)
}
