package main

import (
	"fmt"
	"testing"
)

func TestExpandStones(t *testing.T) {
	want := 22

	cache := make(map[CacheVal]int)
	got := CountStones("125", 6, &cache) + CountStones("17", 6, &cache)
	fmt.Println(got)

	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestPart1(t *testing.T) {
	want := 55312

	got := Part1("./test.txt")

	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
