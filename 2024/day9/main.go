package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	input_file := "./input.txt"
	result := Part1(input_file)
	fmt.Println("Part 1 result: ", result)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func Part1(input_file string) int {
	file, err := os.Open(input_file)
	check(err)
	scanner := bufio.NewScanner(file)

	total := 0
	disk_map := DiskMap{}

	var i, j int

	for scanner.Scan() {
		line := scanner.Text()
		for idx, num := range line {
			num := int(num - '0')
			disk_map.input = append(disk_map.input, num)

			if idx%2 == 0 {
				disk_map.files = append(disk_map.files, DiskBuffer{i, num})
				i++
			} else {
				disk_map.free_space = append(disk_map.free_space, DiskBuffer{j, num})
				j++
			}
		}
	}

	disk_map.dense = disk_map.getDenseRepresentation()

	for idx, num := range disk_map.dense {
		total += num * idx
	}

	return total
}

type DiskMap struct {
	input      []int
	files      []DiskBuffer
	free_space []DiskBuffer
	dense      []int
}

type DiskBuffer struct {
	id     int
	length int
}

func (d *DiskMap) getDenseRepresentation() []int {
	var dense []int

	var i int

	for {
		if len(d.files) == 0 {
			break
		}

		if i%2 == 0 {
			f := d.files[0]

			for range f.length {
				dense = append(dense, f.id)
				f.length--
			}

			d.files = d.files[1:]
		} else {
			// replace free space with last file
			if len(d.free_space) == 0 {
				continue
			}

			free_space := d.free_space[0]
			last_file := d.files[len(d.files)-1]
			d.files = d.files[:len(d.files)-1]

			for range free_space.length {
				// Get the last file with length > 0
				for {
					if last_file.length > 0 {
						// Found a file with length > 0
						break
					} else {
						if len(d.files) > 0 {
							last_file = d.files[len(d.files)-1]
							d.files = d.files[:len(d.files)-1]
						} else {
							// No more files to try
							break
						}
					}
				}

				if last_file.length > 0 {
					dense = append(dense, last_file.id)
					last_file.length--
				}
			}

			if last_file.length > 0 {
				// add the last file back since we're not done with it
				d.files = append(d.files, last_file)
			}

			d.free_space = d.free_space[1:]
		}

		i++
	}

	return dense
}
