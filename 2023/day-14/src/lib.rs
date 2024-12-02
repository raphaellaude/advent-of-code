use std::io::{self, Write};

pub fn part_one(input: &str) -> usize {
    let transposed = roll_boulders(transpose(input.to_string()));

    score_with_rotation(&transposed)
}

fn transpose(input: String) -> String {
    let horiz: Vec<_> = input.lines().map(|l| l.to_string()).collect();

    (0..input.lines().next().unwrap().len())
        .into_iter()
        .map(|idx| {
            let mut s = horiz
                .iter()
                .rev()
                .map(|col| col.chars().nth(idx).unwrap())
                .collect::<String>();
            s.push_str(&"\n");
            s
        })
        .collect::<String>()
}

fn roll_boulders(input: String) -> String {
    input
        .lines()
        .map(|l| {
            let mut s = roll_line(l.to_string());
            s.push_str(&"\n");
            s
        })
        .collect::<String>()
}

fn roll_line(mut line: String) -> String {
    loop {
        line = line.replace("O.", ".O");
        if !line.contains("O.") {
            return line;
        }
    }
}

fn score_with_rotation(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(idx, c)| if c == 'O' { idx + 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn score(input: &str) -> usize {
    input
        .lines()
        .rev()
        .enumerate()
        .map(|(idx, l)| {
            l.chars()
                .map(|c| if c == 'O' { idx + 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut transposed = input.to_string();

    let n = 1_000_000_000;
    let mut cache = Vec::new();

    for idx in 0..n {
        for _ in 0..4 {
            transposed = roll_boulders(transpose(transposed));
        }

        if cache.contains(&transposed) {
            let found_at = cache.iter().position(|r| *r == transposed).unwrap();
            let cycle_len = idx - found_at;

            let n_remaining = n - idx - 1;
            let n_remaining = n_remaining % cycle_len;

            for _ in 0..n_remaining {
                for _ in 0..4 {
                    transposed = roll_boulders(transpose(transposed));
                }
            }
            return score(&transposed);
        }

        cache.push(transposed.clone());

        print!("\r{idx}");
        io::stdout().flush().unwrap();
    }

    println!("Wow! You've made it...!");
    score(&transposed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part_one(&input), 136)
    }

    #[test]
    fn test_roll_line() {
        assert_eq!(roll_line("O.OO#....#".to_string()), ".OOO#....#")
    }

    #[test]
    fn test_part_two() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part_two(&input), 64)
    }
}
