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

func TestSmall(t *testing.T) {
	input_file := "./test_small.txt"
	want := 140

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
