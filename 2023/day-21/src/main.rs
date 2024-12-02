use std::fs;
use template::part_one;

fn main() {
    let input = fs::read_to_string("input.txt").expect("expected valid input");

    let result = part_one(&input, 64);
    println!("Part 1: {result}");

    // let result = part_two(&input);
    // println!("Part 2: {result}");
}
