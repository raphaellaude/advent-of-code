package main

import (
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 36

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMainSimple(t *testing.T) {
	input_file := "./test_simple.txt"
	want := 1

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain_02(t *testing.T) {
	input_file := "./test_02.txt"
	want := 2

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain_03(t *testing.T) {
	input_file := "./test_02.txt"
	want := 3

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain_04(t *testing.T) {
	input_file := "./test_04.txt"
	want := 4

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
