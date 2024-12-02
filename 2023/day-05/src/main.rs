use day_05::{part_one, part_two};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = part_one(&input);
    println!("Hello, world! {result}");

    let result = part_two(&input);
    println!("Hello part 2 {result}");
}
