package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 14

	got, _ := Main(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain2(t *testing.T) {
	input_file := "./test.txt"
	want := 34

	_, got := Main(input_file)
	if got != want {
		t.Errorf("Part2 = %d; want %d", got, want)
	}
}

func TestMain2_T(t *testing.T) {
	input_file := "./test2.txt"
	want := 9

	_, got := Main(input_file)
	if got != want {
		t.Errorf("Part2 = %d; want %d", got, want)
	}
}
