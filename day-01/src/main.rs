use std::fs;
use std::error::Error;

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
    println!("{result}");

    result
}

fn replace_spelled_digits(s: &str) -> String {
    let spelled_digits: Vec<(&str, &str)> = vec![
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

    let mut result = s.to_string();
    let mut matches: Vec<(usize, &str, &str)> = Vec::new();

    for (replacement, digit) in &spelled_digits {
        match s.find(digit) {
            Some(x) => matches.push((x, replacement, digit)),
            None => (),
        }
    }

    matches.sort_by_key(|k| k.0);

    for (_, replacement, digit) in &matches {
        result = result.replace(digit, replacement);
    }

    if matches.len() > 0 {
        println!("{s} -> {result}");
    };

    result
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

    #[test]
    fn test_first_example() {
        let example = "1abc2";

        assert_eq!(12, decode_substring(example));
    }

    #[test]
    fn test_second_example() {
        let example = "pqr3stu8vwx";

        assert_eq!(38, decode_substring(example));
    }

    #[test]
    fn test_third_example() {
        let example = "a1b2c3d4e5f";

        assert_eq!(15, decode_substring(example));
    }

    #[test]
    fn test_fourth_example() {
        let example = "treb7uchet";

        assert_eq!(77, decode_substring(example));
    }

    #[test]
    fn test_part_2_example_1() {
        let example = "two1nine";
        assert_eq!(29, decode_substring(example));
    }
    
    #[test]
    fn test_part_2_example_2() {
        let example = "eightwothree";
        assert_eq!(83, decode_substring(example));
    }
    
    #[test]
    fn test_part_2_example_3() {
        let example = "abcone2threexyz";
        assert_eq!(13, decode_substring(example));
    }

    #[test]
    fn test_part_2_example_4() {
        let example = "xtwone3four";
        assert_eq!(24, decode_substring(example));
    }

    #[test]
    fn test_part_2_example_5() {
        let example = "4nineeightseven2";
        assert_eq!(42, decode_substring(example));
    }

    #[test]
    fn test_part_2_example_6() {
        let example = "zoneight234";
        assert_eq!(14, decode_substring(example));
    }

    #[test]
    fn test_part_2_example_7() {
        let example = "7pqrstsixteen";
        assert_eq!(76, decode_substring(example));
    }
}

