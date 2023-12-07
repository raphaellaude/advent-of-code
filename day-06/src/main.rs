use day_06::{part_one, part_two};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Expected a valid file to read");

    let result = part_one(&input);
    println!("Hello, world! {result}");

    let result = part_two(&input);
    println!("Hello, world! {result}");
}
