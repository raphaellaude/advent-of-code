use std::fs;
use std::ops::Range;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read the file");

    let result = part_two(&input);
    println!("Part 1: {}", result);
}

fn part_two(input: &str) -> i32 {
    let mut iter = input.lines().peekable();
    let mut result = 0;

    let mut line_len = 0;

    let mut previous_line: Option<String> = None;

    while let Some(line) = iter.next() {
        if line_len == 0 {
            line_len = line.len();
        }

        let line = line.to_string();

        for (idx, c) in line.chars().enumerate() {
            if c == '*' {
                let adjacent_numbers =
                    get_adjacent_numbers(idx, &line, line_len, &previous_line, iter.peek());
                dbg!(&adjacent_numbers);
                if adjacent_numbers.len() == 2 {
                    result += &adjacent_numbers[0] * &adjacent_numbers[1];
                }
            }
        }

        previous_line = Some(line);
    }

    result
}

fn get_adjacent_numbers(
    idx: usize,
    line: &String,
    line_len: usize,
    previous_line: &Option<String>,
    next_line: Option<&&str>,
) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    let slice_start = if idx == 0 { 0 } else { idx + 1 };
    let slice_end = if idx == line_len - 1 {
        line_len - 1
    } else {
        idx + 1
    };
    let range = slice_start..slice_end;

    numbers.extend(get_numbers_from_slice(&line, &range, line_len));

    numbers.extend(match &previous_line.clone() {
        Some(l) => get_numbers_from_slice(l, &range, line_len),
        None => vec![],
    });

    numbers.extend(match next_line {
        Some(l) => get_numbers_from_slice(&(l.to_string()), &range, line_len),
        None => vec![],
    });

    numbers
}

fn get_numbers_from_slice(line: &String, range: &Range<usize>, line_len: usize) -> Vec<i32> {
    let mut slice_line_numbers: Vec<i32> = Vec::new();
    let mut number_start: Option<usize> = None;

    for (idx, c) in line.chars().enumerate() {
        let is_num = c.is_numeric();
        if is_num && number_start.is_none() {
            number_start = Some(idx);
        }

        if !is_num && number_start.is_some() {
            let start = number_start.expect("Expected number start");
            if start <= range.end && idx + 1 >= range.start {
                slice_line_numbers.push(line[start..idx].parse::<i32>().unwrap());
            }

            number_start = None;
        } else if (!is_num || (idx + 1 == line_len && is_num)) && number_start.is_some() {
            let start = number_start.expect("Expected number start");
            if start <= range.end && idx + 1 >= range.start {
                slice_line_numbers.push(line[start..].parse::<i32>().unwrap());
            }
        }
    }
    slice_line_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_two(input), 467835);
    }
}
