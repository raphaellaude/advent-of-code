package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 3749

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
