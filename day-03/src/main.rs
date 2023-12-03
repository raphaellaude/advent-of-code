use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read the file");

    let result = part_one(&input);
    println!("Part 1: {}", result);
}

fn part_one(input: &str) -> i32 {
    let mut iter = input.lines().peekable();
    let mut result = 0;

    let mut line_len = 0;

    let mut previous_line: Option<String> = None;

    while let Some(line) = iter.next() {
        let mut to_add: Vec<i32> = Vec::new();

        if line_len == 0 {
            line_len = line.len();
        }

        let line = line.to_string();

        let mut number_start: Option<usize> = None;

        for (idx, c) in line.chars().enumerate() {
            let is_num = c.is_numeric();
            if is_num && number_start.is_none() {
                number_start = Some(idx);
            }

            if !is_num && number_start.is_some() {
                let start = number_start.expect("Expected number start");
                let number = &line[start..idx].parse::<i32>().unwrap();

                let slice_start = if start == 0 { start } else { start - 1 };
                let slice_end = if idx + 1 > line_len {
                    line_len
                } else {
                    idx + 1
                };

                if add_value(&line, slice_start, slice_end, &previous_line, iter.peek()) {
                    to_add.push(*number);
                    result += number;
                };

                number_start = None;
            } else if (!is_num || (idx + 1 == line_len && is_num)) && number_start.is_some() {
                let start = number_start.expect("Expected number start");
                let number = &line[start..].parse::<i32>().unwrap();

                let slice_start = if start == 0 { start } else { start - 1 };
                let slice_end = if idx + 1 > line_len {
                    line_len
                } else {
                    idx + 1
                };

                if add_value(&line, slice_start, slice_end, &previous_line, iter.peek()) {
                    to_add.push(*number);
                    result += number;
                };
            }
        }
        // println!("Adding {:?}", to_add);
        previous_line = Some(line);
    }

    result
}

fn add_value(
    line: &str,
    slice_start: usize,
    slice_end: usize,
    previous_line: &Option<String>,
    next_line: Option<&&str>,
) -> bool {
    if any_symbol(&line[slice_start..slice_end]) {
        return true;
    } else {
        match &previous_line.clone() {
            Some(l) => {
                if any_symbol(&l[slice_start..slice_end]) {
                    return true;
                }
            }
            None => (),
        }

        return match next_line {
            Some(l) => return any_symbol(&l[slice_start..slice_end]),
            None => false,
        };
    }
}

fn any_symbol(str_slice: &str) -> bool {
    str_slice.chars().any(|c| !c.is_numeric() && c != '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
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
        assert_eq!(part_one(input), 4361);
    }

    #[test]
    fn test_part_one_handles_above_and_below() {
        // added '%' below 592
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
...%..755.
...$.*....
.664.598..";
        assert_eq!(part_one(input), 4361);
    }

    #[test]
    fn test_part_one_handles_eol() {
        // added '%' below 592
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
...%..755.
...$.*...$
.664.598.1";
        assert_eq!(part_one(input), 4362);
    }
}
