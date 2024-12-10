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

	starts := []Cell{}

	grounds := make(map[Cell]int)
	var x, y int
	var p1, p2 int

	for scanner.Scan() {
		line := scanner.Text()

		x = 0
		for _, r := range line {
			cell := Cell{x, y}

			if r != '.' {
				grounds[cell] = int(r - '0')
			}

			if r == '0' {
				starts = append(starts, cell)
			}

			x++
		}
		y++
	}

	grid := Grid{rows: y, columns: x}

	for _, s := range starts {
		peaks_found := s.Climb(&grid, grounds, 9)
		// fmt.Printf("%d: found %d peaks\n", idx, len(peaks_found))
		p1 += len(peaks_found)
	}

	for _, s := range starts {
		unique_trails_found := s.GetUniqueTrails(&grid, grounds, 9)
		// fmt.Printf("%d: found %d peaks\n", idx, len(peaks_found))
		p2 += len(unique_trails_found)
	}

	return p1, p2
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

func (grid *Grid) RookNeighbors(c *Cell) []Cell {
	neighbors := []Cell{}
	poses := [][]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}

	for _, i := range poses {
		x, y := i[0], i[1]
		neigh := Cell{x: c.x - x, y: c.y - y}
		if !grid.IsOffMap(&neigh) {
			neighbors = append(neighbors, neigh)
		}
	}

	return neighbors
}

func (cell *Cell) Climb(grid *Grid, grounds map[Cell]int, target int) map[Cell]bool {
	peaks_found := make(map[Cell]bool)

	neighbors := grid.RookNeighbors(cell)
	c := grounds[*cell]

	for _, neighbor := range neighbors {
		t, ok := grounds[neighbor]

		if !ok || t <= c {
			continue
		}

		if t == target && c == target-1 {
			peaks_found[neighbor] = true
			continue
		}

		if t == c+1 {
			new_peaks_found := neighbor.Climb(grid, grounds, target)
			for found := range new_peaks_found {
				peaks_found[found] = true
			}
		}
	}

	return peaks_found
}

func (cell *Cell) GetUniqueTrails(grid *Grid, grounds map[Cell]int, target int) [][]Cell {
	trails_found := [][]Cell{}

	trail := []Cell{*cell}

	neighbors := grid.RookNeighbors(cell)
	c := grounds[*cell]

	for _, neighbor := range neighbors {
		t, ok := grounds[neighbor]

		if !ok || t <= c {
			continue
		}

		if t == target && c == target-1 {
			trail = append(trail, neighbor)
			trails_found = append(trails_found, trail)
			continue
		}

		if t == c+1 {
			trail = append(trail, neighbor)
			new_trails_found := neighbor.GetUniqueTrails(grid, grounds, target)
			for _, trail_found := range new_trails_found {
				full_trail := append(trail, trail_found...)
				trails_found = append(trails_found, full_trail)
			}
		}
	}

	return trails_found
}
