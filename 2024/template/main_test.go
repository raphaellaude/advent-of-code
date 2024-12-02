package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	got := Part1(input_file)
	if got != 15 {
		t.Errorf("Part1 = %d; want 15", got)
	}
}
