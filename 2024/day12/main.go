package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	// Part 1
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

	grounds, columns, rows, queue := BuildGarden(scanner)

	geoms := FindGeometries(grounds, columns, rows, queue)

	for _, g := range geoms {
		p1 += Price(g)
	}

	return p1
}

type Point struct {
	x int
	y int
}

type Ring struct {
	points []Point
}

type Polygon struct {
	exterior  *Ring
	interiors []*Polygon
}

type Geometry interface {
	area() int
	perimiter() int
}

func (p Ring) perimiter() int {
	var perim int

	for i := 0; i < len(p.points)-1; i++ {
		a := p.points[i]
		b := p.points[i+1]
		perim += Abs(b.x-a.x) + Abs(b.y-a.y)
	}

	return perim
}

func (p Polygon) perimiter() int {
	perim := len(p.exterior.points)

	for interior := range p.interiors {
		perim += p.interiors[interior].perimiter()
	}

	return perim
}

func (p Ring) area() int {
	area := 0
	for i := 0; i < len(p.points)-1; i++ {
		j := i + 1
		area += p.points[i].x*p.points[j].y - p.points[j].x*p.points[i].y
	}
	return Abs(area) / 2
}

func (p Polygon) area() int {
	area := p.exterior.area()

	for interior := range p.interiors {
		area -= p.interiors[interior].area()
	}

	return area
}

func Price(p Geometry) int {
	return p.area() * p.perimiter()
}

func Abs[T int](i T) T {
	if i < 0 {
		return -i
	}
	return i
}

func BuildGarden(scanner *bufio.Scanner) (map[Point]rune, int, int, []Point) {
	grounds := make(map[Point]rune)
	queue := []Point{}
	var x, y int

	for scanner.Scan() {
		line := scanner.Text()

		x = 0
		for _, r := range line {
			cell := Point{x, y}
			queue = append(queue, cell)

			if r != '.' {
				grounds[cell] = r
			}

			x++
		}
		y++
	}

	return grounds, x, y, queue
}

func FindGeometries(grounds map[Point]rune, columns int, rows int, queue []Point) []Geometry {
	geoms := []Geometry{}
	// rings := []Ring{}
	visited := make(map[Point]bool)

	for _, point := range queue {
		r := grounds[point]
		neighs := point.TopRookNeighbors(columns, rows)

		check_point := true

		for _, neigh := range neighs {
			// fmt.Println(point, string(r), neigh, visited[neigh], string(grounds[neigh]))
			if grounds[neigh] == r && visited[neigh] {
				check_point = false
				break
			}
		}

		if check_point {
			polygon := GetPolygon(point, grounds, columns, rows, &visited)
			geoms = append(geoms, polygon)
		}
	}

	return geoms
}

type Direction int

const (
	up Direction = iota
	left
	down
	right
)

func (d Direction) Left() Direction {
	return (d + 1) % 4
}

func (d Direction) Right() Direction {
	return (d + 3) % 4
}

var dirs = [4][2]int{{0, -1}, {-1, 0}, {0, 1}, {1, 0}}

func (p Point) nextPos(d Direction) Point {
	vec := dirs[d]
	return Point{p.x + vec[0], p.y + vec[1]}
}

func (p Point) isOffMap(columns int, rows int) bool {
	if p.x < 0 || p.y < 0 {
		return true
	}

	return p.x > columns || p.y > rows
}

func (c Point) TopRookNeighbors(cols int, rows int) []Point {
	neighbors := []Point{}
	poses := [][]int{{0, -1}, {-1, 0}, {1, 0}}

	for _, i := range poses {
		x, y := i[0], i[1]
		neigh := Point{x: c.x + x, y: c.y + y}
		if !neigh.isOffMap(cols, rows) {
			neighbors = append(neighbors, neigh)
		}
	}

	return neighbors
}

var add_points = [4][2]int{{1, 1}, {1, 0}, {0, 0}, {0, 1}}

func GetPolygon(start Point, grounds map[Point]rune, columns int, rows int, visited *map[Point]bool) Ring {
	ring := Ring{points: []Point{}}
	var direction Direction = 1 // start going left
	position := start
	r := grounds[start]

	max_steps := 100
	var i int

	for {
		if i >= max_steps {
			break
		}

		(*visited)[position] = true

		// fmt.Println(position, direction)
		if position == start && direction == left && len(ring.points) != 0 {
			ring.points = append(ring.points, ring.points[0])
			break
		}

		i++

		to_right := position.nextPos(direction.Right())
		if v, ok := grounds[to_right]; ok && v == r && !to_right.isOffMap(columns, rows) {
			position = to_right
			direction = direction.Right()
			ap := add_points[direction]
			x, y := ap[0], ap[1]
			ring.points = append(ring.points, Point{position.x + x, position.y + y})
			continue
		}

		straight := position.nextPos(direction)
		if v, ok := grounds[straight]; ok && v == r && !straight.isOffMap(columns, rows) {
			position = straight
		} else {
			direction = direction.Left()
			ap := add_points[direction]
			x, y := ap[0], ap[1]
			ring.points = append(ring.points, Point{position.x + x, position.y + y})
		}
	}

	fmt.Println(ring)

	// fmt.Println("interior points", interior_points)

	// if len(interior_points) > 0 {
	// 	interior_visits := make(map[Point]bool)

	// 	for _, point := range interior_points {
	// 		r := grounds[point]
	// 		neighs := point.TopRookNeighbors(columns, rows)

	// 		check_point := true

	// 		for _, neigh := range neighs {
	// 			if grounds[neigh] == r && interior_visits[neigh] {
	// 				check_point = false
	// 				break
	// 			}
	// 		}

	// 		if check_point {
	// 			interior_ring := GetPolygon(point, grounds, columns, rows, &interior_visits)
	// 			poly.interiors = append(poly.interiors, &interior_ring)
	// 		}
	// 	}
	// }

	return ring
}
