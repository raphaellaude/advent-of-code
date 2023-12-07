use day_07::part_one;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("expected valid input");
    let result = part_one(&input);

    println!("Hello, world! {result}");
}
