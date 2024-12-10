package main

import (
	"fmt"
	"testing"
)

func TestMain(t *testing.T) {
	input_file := "./test.txt"
	want := 36
	fmt.Println("TEST Main")

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMainSimple(t *testing.T) {
	input_file := "./test_simple.txt"
	want := 1
	fmt.Println("TEST Simple")

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain_02(t *testing.T) {
	input_file := "./test_02.txt"
	want := 2
	fmt.Println("TEST 2")

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain_03(t *testing.T) {
	input_file := "./test_03.txt"
	want := 3
	fmt.Println("TEST 3")

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestMain_04(t *testing.T) {
	input_file := "./test_04.txt"
	want := 4
	fmt.Println("TEST 4")

	got := Part1(input_file)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}
