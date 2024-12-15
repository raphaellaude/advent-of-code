package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 1930

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestInteriors(t *testing.T) {
	input_file := "./test_interiors.txt"
	want := 4*4 + 21*(20+16)

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestGetPolygonA(t *testing.T) {
	start := Point{0, 0}
	basicMap := map[Point]rune{{0, 0}: 'A', {1, 0}: 'A', {0, 1}: 'A', {1, 1}: 'B'}
	cols, rows := 2, 2
	// want := Ring{points: []Point{}}
	visited := make(map[Point]bool)

	got := GetPolygon(start, basicMap, cols, rows, &visited)

	if len(got.points) != 7 {
		t.Errorf("Got %d", len(got.points))
	}
}

func TestGetPolygonB(t *testing.T) {
	start := Point{1, 1}
	basicMap := map[Point]rune{{0, 0}: 'A', {1, 0}: 'A', {0, 1}: 'A', {1, 1}: 'B'}
	cols, rows := 2, 2
	// want := Ring{points: []Point{}}
	visited := make(map[Point]bool)

	got := GetPolygon(start, basicMap, cols, rows, &visited)

	if len(got.points) != 5 {
		t.Errorf("Got %d", len(got.points))
	}
}

func TestGetPolygonC(t *testing.T) {
	start := Point{1, 1}
	basicMap := map[Point]rune{
		{0, 0}: 'X',
		{0, 2}: 'X',
		{2, 0}: 'X',
		{2, 2}: 'X',
		{1, 0}: 'C',
		{1, 1}: 'C',
		{1, 2}: 'C',
		{0, 1}: 'C',
		{2, 1}: 'C',
	}
	cols, rows := 2, 2
	// want := Ring{points: []Point{}}
	visited := make(map[Point]bool)

	got := GetPolygon(start, basicMap, cols, rows, &visited)

	if len(got.points) != 13 {
		t.Errorf("Got %d", len(got.points))
	}
}
