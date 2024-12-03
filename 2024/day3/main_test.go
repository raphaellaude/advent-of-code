package main

import (
	"testing"
)

func TestPar1(t *testing.T) {
	input_file := "./test.txt"
	want := 161

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestPar2(t *testing.T) {
	input_file := "./test2.txt"
	want := 48

	got := Part2(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
