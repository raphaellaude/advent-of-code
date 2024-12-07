package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 41

	got, _ := Main(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	input_file := "./test.txt"
	want := 6

	_, got := Main(input_file)
	if got != want {
		t.Errorf("Part2 = %d; want %d", got, want)
	}
}
