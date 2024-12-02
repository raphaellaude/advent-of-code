use std::fs;
use template::{part_one, part_two};

fn main() {
    let input = fs::read_to_string("input.txt").expect("expected valid input");

    let result = part_one(&input);
    println!("Part 1: {result}");

    let result = part_two(&input, 1000000);
    println!("Part 2: {result}");
}
