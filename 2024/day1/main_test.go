package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 11

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	input_file := "./test.txt"
	want := 31

	got := Part2(input_file)
	if got != want {
		t.Errorf("Part2 = %d; want %d", got, want)
	}
}
