package main

import (
	"testing"
)

func TestPart1(t *testing.T) {
	input_file := "./test.txt"
	want := 2

	got := CheckReports(input_file, 0)
	if got != want {
		t.Errorf("Part1 = %d; want %d", got, want)
	}
}

func TestPart2(t *testing.T) {
	input_file := "./test.txt"
	want := 4

	got := CheckReports(input_file, 1)
	if got != want {
		t.Errorf("Part2 = %d; want %d", got, want)
	}
}

func TestIsValidEnding(t *testing.T) {
	nums := []string{"2", "4", "6", "9", "10", "9"}
	tolerance := 1
	want := true

	got := IsValid(nums, tolerance)
	if got != want {
		t.Errorf("IsValidEnding = %t; want %t", got, want)
	}
}

func TestIsValidFirstStepIssue(t *testing.T) {
	nums := []string{"2", "2", "3", "4", "5", "6"}
	tolerance := 1
	want := true

	got := IsValid(nums, tolerance)
	if got != want {
		t.Errorf("IsValidEnding = %t; want %t", got, want)
	}
}

func TestIsValidUpAndDownIssue(t *testing.T) {
	nums := []string{"2", "4", "3", "4", "5"}
	tolerance := 1
	want := true

	got := IsValid(nums, tolerance)
	if got != want {
		t.Errorf("IsValidEnding = %t; want %t", got, want)
	}
}
