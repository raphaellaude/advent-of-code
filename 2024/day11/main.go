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
		vals := strings.Split(line, " ")

		for _, val := range vals {
			stones := Stones{&Stone{val, nil}}
			stones = *stones.Expand(25)
			total += stones.len()
		}
	}

	return total
}

type Stone struct {
	value string
	next  *Stone
}

type Stones struct {
	head *Stone
}

func (stones *Stones) Expand(steps int) *Stones {
	for i := 0; i < steps; i++ {
		stone := stones.head

		for {
			if stone == nil {
				break
			}

			if stone.value == "0" {
				stone.value = "1"
				stone = stone.next
				continue
			}

			if len(stone.value)%2 == 0 {
				stone = stone.Split().next
			} else {
				vint, err := strconv.Atoi(stone.value)
				check(err)
				vint *= 2024
				stone.value = strconv.Itoa(vint)
				stone = stone.next
			}
		}

	}

	return stones
}

func (stone *Stone) Split() Stone {
	l := len(stone.value)
	new_value := stone.value[:l/2]
	new_stone_value, err := strconv.Atoi(stone.value[l/2:])
	check(err)

	stone.value = new_value
	new_stone := Stone{value: strconv.Itoa(new_stone_value), next: stone.next}
	stone.next = &new_stone

	return new_stone
}

func (stones *Stones) len() int {
	stone := stones.head
	total := 0
	for {
		if stone == nil {
			break
		}

		total++
		stone = stone.next
	}

	return total
}

func (stones *Stones) collect() []string {
	values := []string{}

	stone := stones.head
	for {
		if stone == nil {
			break
		}

		values = append(values, stone.value)
		stone = stone.next
	}

	return values
}
