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

	var p1, _ int

	grounds := make(map[Point]rune)
	var x, y int

	for scanner.Scan() {
		line := scanner.Text()
		x = 0
		for _, r := range line {
			cell := Point{x, y}
			grounds[cell] = r
			x++
		}
		y++
	}

	visited := make(map[Point]bool)

	for point, r := range grounds {
		queue := Queue[Point]{}

		if !visited[point] {
			queue.Add(point)
		} else {
			continue
		}

		var area int
		var perim int

		for {
			if len(queue.values) == 0 {
				break
			}

			pt := queue.PopLeft()

			if visited[pt] {
				continue
			}

			area++
			visited[pt] = true

			neighbors := pt.RookNeighbors()
			for _, neigh := range neighbors {
				has_visited := visited[neigh]
				v, ok := grounds[neigh]

				if ok && v == r {
					if !has_visited {
						queue.Add(neigh)
					}
				} else {
					perim++
				}
			}
		}

		p1 += area * perim
	}

	return p1
}

type Queue[T any] struct {
	values []T
}

func (queue *Queue[T]) PopLeft() T {
	val := queue.values[0]
	(*queue).values = queue.values[1:]
	return val
}

func (queue *Queue[T]) Add(t T) {
	queue.values = append(queue.values, t)
}

type Point struct {
	x int
	y int
}

var directions = [][]int{{0, -1}, {-1, 0}, {1, 0}, {0, 1}}

func (c Point) RookNeighbors() []Point {
	neighbors := []Point{}

	for _, i := range directions {
		x, y := i[0], i[1]
		neighbors = append(neighbors, Point{x: c.x + x, y: c.y + y})
	}

	return neighbors
}
