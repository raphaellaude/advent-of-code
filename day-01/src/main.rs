use std::fs;
use std::error::Error;

const SPELLED_DIGITS: &[(&str, &str)] = &[
    ("1", "one"),
    ("2", "two"),
    ("3", "three"),
    ("4", "four"),
    ("5", "five"),
    ("6", "six"),
    ("7", "seven"),
    ("8", "eight"),
    ("9", "nine"),
];

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("encoded_calibration_doc.txt")?;
    
    let total: i32 = contents.lines()
        .map(|line| decode_substring(line))
        .sum();

    println!("The sum of calibration values is {total}!");

    Ok(())
}

fn decode_substring(s: &str) -> i32 {
    let decoded = first_last_numeric(
        replace_spelled_digits(&s)
    );
    let result: i32 = decoded.parse().unwrap();

    result
}

fn replace_spelled_digits(s: &str) -> String {
    let mut result = s.to_string();

    let matches = get_spelled_matches(s);
    if matches.len() > 0 {
        let (_, replacement, digit) = matches[0]; 
        result = result.replace(digit, replacement);
    }

    let matches = get_spelled_matches(&result);
    if matches.len() > 0 {
        let (_, replacement, digit) = matches[matches.len() - 1]; 
        result = result.replace(digit, replacement);
    }

    result
}

fn get_spelled_matches(s: &str) -> Vec<(usize, &str, &str)> {
    let mut matches: Vec<(usize, &str, &str)> = Vec::new();

    for (replacement, digit) in SPELLED_DIGITS {
        match s.find(digit) {
            Some(x) => matches.push((x, replacement, digit)),
            None => (),
        }
    }

    matches.sort_by_key(|k| k.0);

    matches
}

fn first_last_numeric(s: String) -> String {
    let num_chars: Vec<char> = s.chars()
        .filter(|c| c.is_numeric())
        .collect();

    let first_num = num_chars[0];
    let last_num = num_chars[num_chars.len() - 1];

    let result = format!("{first_num}{last_num}");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn part_one(
        #[case] line: &str,
        #[case] expected: i32
    ) {
        assert_eq!(expected, decode_substring(line));
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("two9jzgzbtwonef", 21)]
    fn part_two(
        #[case] line: &str,
        #[case] expected: i32
    ) {
        assert_eq!(expected, decode_substring(line));
    }
}

