use day_04::part_one;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let part1_result = part_one(&input);
    println!("Part 1: {part1_result}");
}
