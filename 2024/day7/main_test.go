package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input_file := "./test.txt"
	want := 3749

	got, _ := Main(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	input_file := "./test.txt"
	want := 11387

	_, got := Main(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
