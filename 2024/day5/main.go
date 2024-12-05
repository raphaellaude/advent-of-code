package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/dominikbraun/graph"
)

func main() {
	input_file := "./input.txt"
	part1, part2 := Main(input_file)
	fmt.Println("Part 1 result: ", part1)
	fmt.Println("Part 2 result: ", part2)
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

	part1 := 0
	part2 := 0
	g := graph.New(graph.StringHash, graph.Directed(), graph.Acyclic())

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			continue
		}

		if strings.Contains(line, "|") {
			vals := strings.Split(line, "|")
			i, j := vals[0], vals[1]

			g.AddVertex(i)
			g.AddVertex(j)
			g.AddEdge(i, j)
			continue
		}

		nodes := strings.Split(line, ",")
		end := nodes[len(nodes)-1]
		middle := nodes[len(nodes)/2]

		violation := false

		for idx, start := range nodes {
			if start == end {
				break
			}

			_, err = g.Edge(nodes[idx+1], start)

			if err == nil {
				violation = true
				break
			}
		}

		if !violation {
			val, err := strconv.Atoi(middle)
			check(err)
			part1 += val
		} else {
			nodes = FixOrdering(nodes, g)

			val, err := strconv.Atoi(nodes[len(nodes)/2])
			check(err)

			part2 += val
		}
	}

	return part1, part2
}

func FixOrdering(nodes []string, g graph.Graph[string, string]) []string {
	broken := false

	for idx, start := range nodes {
		if start == nodes[len(nodes)-1] {
			break
		}

		_, err := g.Edge(nodes[idx+1], start)

		if err == nil {
			tmp := nodes[idx+1]
			nodes[idx+1] = start
			nodes[idx] = tmp
			broken = true
		}
	}

	if broken {
		return FixOrdering(nodes, g)
	}

	return nodes
}
