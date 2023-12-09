use day_10::{part_one, part_two};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("expected valid input");

    let result = part_one(&input);
    println!("Part 1: {result}");

    let result = part_two(&input);
    println!("Part 2: {result}");
}
