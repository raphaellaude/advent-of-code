pub fn part_one(input: &str) -> usize {
    let parsed = parse(input);

    parsed
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .map(|(idx, c)| if c == 'O' { idx } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: &str) -> String {
    let horiz: Vec<_> = input.lines().map(|l| l.to_string()).collect();

    (0..input.lines().next().unwrap().len())
        .into_iter()
        .map(|idx| {
            format!(
                "{:?}\n",
                roll_boulders(
                    horiz
                        .iter()
                        .rev()
                        .map(|col| col.chars().nth(idx).unwrap())
                        .collect::<String>()
                )
            )
        })
        .collect::<String>()
}

fn roll_boulders(mut line: String) -> String {
    loop {
        line = line.replace("O.", ".O");
        if !line.contains("O.") {
            return line;
        }
    }
}

pub fn part_two(_input: &str) -> usize {
    todo!()
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
    fn test_roll_boulders() {
        assert_eq!(roll_boulders("O.OO#....#".to_string()), ".OOO#....#")
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_one(&input), 0)
    // }
}
