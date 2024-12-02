package aoc

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 0

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
