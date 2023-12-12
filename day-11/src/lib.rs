use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    let galactic_map = parse(input, 1);
    get_total_distance(galactic_map)
}

pub fn part_two(input: &str, factor: usize) -> usize {
    let galactic_map = parse(input, factor);
    get_total_distance(galactic_map)
}

struct Cell {
    is_galaxy: bool,
    x: usize,
    y: usize,
}

fn get_total_distance(galactic_map: HashMap<usize, Cell>) -> usize {
    galactic_map
        .values()
        .into_iter()
        .filter(|c| c.is_galaxy)
        .combinations(2)
        .map(|cells| {
            let a = cells[0];
            let b = cells[1];

            let vert_dist = if b.y > a.y { b.y - a.y } else { a.y - b.y };
            let hztl_dist = if b.x > a.x { b.x - a.x } else { a.x - b.x };

            vert_dist + hztl_dist
        })
        .sum()
}

fn parse(input: &str, factor: usize) -> HashMap<usize, Cell> {
    let mut galactic_map: HashMap<usize, Cell> = HashMap::new();

    let data: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => 0,
                })
                .collect()
        })
        .collect();

    let (empty_rows, empty_cols) = get_galactic_voids(&data);

    let line_len = data[0].len();

    data.iter().enumerate().for_each(|(row_num, row_vals)| {
        row_vals.iter().enumerate().for_each(|(col_num, col_val)| {
            let pos = row_num * line_len + col_num;
            let add_cols: usize = empty_cols.iter().map(|v| (*v < col_num) as usize).sum();
            let add_rows: usize = empty_rows.iter().map(|v| (*v < row_num) as usize).sum();

            let sub = if factor == 1 { 0 } else { 1 };
            let x = col_num + add_cols * (factor - sub);
            let y = row_num + add_rows * (factor - sub);

            galactic_map.insert(
                pos,
                Cell {
                    is_galaxy: *col_val == 1,
                    x,
                    y,
                },
            );
        })
    });

    galactic_map
}

fn get_galactic_voids(data: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows: Vec<usize> = data
        .iter()
        .enumerate()
        .filter(|(_, v)| v.iter().all(|v| *v == 0))
        .map(|(idx, _)| idx)
        .collect();

    let empty_cols: Vec<usize> = (0..data.len())
        .into_iter()
        .filter(|idx| data.iter().all(|r| r[*idx] == 0))
        .collect();

    (empty_rows, empty_cols)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part_one(&input), 374)
    }

    #[test]
    fn test_part_two_10() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part_two(&input, 10), 1030)
    }

    #[test]
    fn test_part_two_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(part_two(&input, 100), 8410)
    }
}
