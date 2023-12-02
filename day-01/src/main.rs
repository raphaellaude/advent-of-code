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

    result
}

fn replace_spelled_digits(s: &str) -> String {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &s[index..];

        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };
        index += 1;
        result
    });
    
    let out = line_iter.collect();

    out
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

