package main

import (
	"fmt"
	"testing"
)

func TestExpandStones(t *testing.T) {
	want := 22

	stones := Stones{&Stone{"125", &Stone{"17", nil}}}

	stones = *stones.Expand(6)
	got := stones.collect()
	fmt.Println(got)

	if len(got) != want {
		t.Errorf("Part1 = %d; want %d", len(got), want)
	}
}

func TestPart1(t *testing.T) {
	want := 55312

	got := Part1("./test.txt")

	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
