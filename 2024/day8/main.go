package main

import (
	"bufio"
	"fmt"
	"os"
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

	antinodes := make(map[Cell]bool)
	resonant_antinodes := make(map[Cell]bool)

	var y int
	var x int

	grounds := make(map[Cell]Tower)
	towers := make(map[rune][]Tower)

	for scanner.Scan() {
		line := scanner.Text()

		x = 0
		for _, r := range line {
			cell := Cell{x, y}
			grounds[cell] = Tower{cell, r}
			if r != '.' {
				towers[r] = append(towers[r], Tower{cell, r})
			}
			x++
		}
		y++
	}

	grid := Grid{rows: y, columns: x}

	for _, ts := range towers {
		for _, ts := range combinations(ts, 2) {
			t1, t2 := ts[0], ts[1]
			if t1.cell != t2.cell {
				a1, a2 := t1.cell.Antinodes(&t2.cell)
				if !grid.IsOffMap(&a1) {
					antinodes[a1] = true
				}
				if !grid.IsOffMap(&a2) {
					antinodes[a2] = true
				}

				resonant_antinodes[t1.cell] = true
				resonant_antinodes[t2.cell] = true

				for _, a := range t1.cell.ResonantAntinodes(&t2.cell, &grid) {
					if !grid.IsOffMap(&a) {
						resonant_antinodes[a] = true
					}
				}
			}
		}
	}

	// grid.PrintAntinodes(resonant_antinodes)

	return len(antinodes), len(resonant_antinodes)
}

type Grid struct {
	rows    int
	columns int
}

type Cell struct {
	x int
	y int
}

func (grid *Grid) IsOffMap(c *Cell) bool {
	return c.x >= grid.columns || c.y >= grid.rows || c.x < 0 || c.y < 0
}

func (c *Cell) Antinodes(pair *Cell) (Cell, Cell) {
	y_diff := c.y - pair.y
	x_diff := c.x - pair.x

	antinode_1 := Cell{c.x + x_diff, c.y + y_diff}
	antinode_2 := Cell{pair.x - x_diff, pair.y - y_diff}
	return antinode_1, antinode_2
}

func (c *Cell) ResonantAntinodes(pair *Cell, grid *Grid) []Cell {
	y_diff := c.y - pair.y
	x_diff := c.x - pair.x

	num_x := grid.columns/Abs(x_diff) + 1
	num_y := grid.rows/Abs(y_diff) + 1

	antinodes := []Cell{}

	for i := 1; i <= num_x; i++ {
		for j := 1; j <= num_y; j++ {
			scale := i * j
			antinode_1 := Cell{c.x + x_diff*scale, c.y + y_diff*scale}
			antinode_2 := Cell{pair.x - x_diff*scale, pair.y - y_diff*scale}
			antinodes = append(antinodes, antinode_1, antinode_2)
		}
	}
	return antinodes
}

func (grid *Grid) PrintAntinodes(antinodes map[Cell]bool) {
	for y := 0; y < grid.rows; y++ {
		for x := 0; x < grid.columns; x++ {
			if antinodes[Cell{x, y}] {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

type SignedNumber interface {
	int | int8 | int16 | int32 | int64 |
		float32 | float64
}

func Abs[V SignedNumber](i V) V {
	if i < 0 {
		return -i
	}
	return i
}

type Tower struct {
	cell Cell
	r    rune
}

func combinations[T any](slice []T, k int) [][]T {
	result := [][]T{}

	for i := 0; i < (1 << len(slice)); i++ {
		subset := []T{}
		for j := 0; j < len(slice); j++ {
			if (i>>j)&1 == 1 {
				subset = append(subset, slice[j])
			}
		}

		if len(subset) == k {
			result = append(result, subset)
		}
	}

	return result
}
