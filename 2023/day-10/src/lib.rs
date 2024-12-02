use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let line_len = input.lines().next().unwrap().len();

    let path_lens: Vec<usize> = get_longest_paths(input, line_len)
        .iter()
        .map(|p| p.len())
        .collect();

    let distance = *path_lens.iter().max().unwrap() as u32;

    if distance % 2 == 0 {
        return distance / 2;
    }
    distance / 2 + 1
}

fn get_longest_paths(input: &str, line_len: usize) -> Vec<Vec<usize>> {
    let (s_loc, hash_map) = parse_input(input, line_len);
    let mut longest_path: Vec<usize> = Vec::new();

    let first_moves = get_first_moves(s_loc, line_len);

    let mut paths: Vec<Vec<usize>> = Vec::new();

    for first_move in first_moves {
        let mut prev_loc = s_loc;
        let mut current_loc = Some(first_move);

        while current_loc.is_some() {
            let node = current_loc.unwrap();
            let kind = *hash_map.get(&node).unwrap();

            if kind == 'S' {
                break;
            }

            current_loc = next_loc(node, prev_loc, kind, line_len);

            prev_loc = node;
            longest_path.push(node);
        }

        longest_path.push(s_loc);

        paths.push(longest_path.clone());
        longest_path.clear();
    }

    paths
}

fn get_first_moves(starting_pos: usize, line_len: usize) -> Vec<usize> {
    let mut neighbors = vec![starting_pos + 1, starting_pos + line_len];

    if starting_pos >= line_len {
        neighbors.push(starting_pos - line_len);
    } else if starting_pos > 0 {
        neighbors.push(starting_pos - 1);
    }

    neighbors
}

pub fn part_two(input: &str) -> i64 {
    let line_len = input.lines().next().unwrap().len();

    let longest_paths: Vec<Vec<usize>> = get_longest_paths(input, line_len);
    let max_path_len: usize = longest_paths.iter().map(|p| p.len()).max().unwrap();

    let longest_paths: Vec<&Vec<usize>> = longest_paths
        .iter()
        .filter(|p| p.len() == max_path_len)
        .collect();

    let longest_path: Vec<usize> = longest_paths[0].clone();

    let mut points = longest_path
        .iter()
        .map(|p| Point {
            x: (p % line_len) as i64,
            y: (p / line_len) as i64,
        })
        .collect::<Vec<Point>>();
    points.push(points[0].clone());

    // dbg!(&points);
    dbg!(_shoelace_area(&points));
    dbg!(picks_area(&points));
    dbg!(&points.len());

    picks_area(&points) - (points.len() - 1) as i64
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

fn _shoelace_area(points: &Vec<Point>) -> i64 {
    points
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>()
        / 2
}

fn picks_area(points: &Vec<Point>) -> i64 {
    points
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x + (b.x - a.x + b.y - a.y).abs())
        .sum::<i64>()
        / 2
        + 1
}

fn parse_input(input: &str, line_len: usize) -> (usize, HashMap<usize, char>) {
    let mut starting_position: usize = 0;
    let mut hash_map: HashMap<usize, char> = HashMap::new();

    input.lines().enumerate().for_each(|(line_no, l)| {
        l.chars().enumerate().for_each(|(char_no, kind)| {
            let loc = line_len * line_no + char_no;
            hash_map.insert(loc, kind);

            if kind == 'S' {
                starting_position = loc;
            }
        })
    });

    if starting_position == 0 {
        panic!("Probably couln't determine starting position!");
    }

    (starting_position, hash_map)
}

fn next_loc(loc: usize, prev_loc: usize, kind: char, line_len: usize) -> Option<usize> {
    return match kind {
        'J' => {
            if loc == prev_loc + 1 {
                Some(loc - line_len)
            } else {
                Some(loc - 1)
            }
        }
        '|' => {
            if loc == prev_loc + line_len {
                Some(loc + line_len)
            } else {
                Some(loc - line_len)
            }
        }
        'L' => {
            if loc == prev_loc - 1 {
                Some(loc - line_len)
            } else {
                Some(loc + 1)
            }
        }
        '-' => {
            if loc == prev_loc + 1 {
                Some(loc + 1)
            } else {
                Some(loc - 1)
            }
        }
        '7' => {
            if loc == prev_loc + 1 {
                Some(loc + line_len)
            } else {
                Some(loc - 1)
            }
        }
        'F' => {
            if loc == prev_loc - 1 {
                Some(loc + line_len)
            } else {
                Some(loc + 1)
            }
        }
        '.' => None,
        'S' => panic!("Should not reach `S`"),
        _ => panic!("Could not match character!"),
    };
}

#[cfg(test)]
mod test {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn test_part_one_simple_loop() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part_one(&input), 4)
    }

    #[test]
    fn test_part_one_bigger_loop() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part_one(&input), 8)
    }

    #[test]
    fn test_part_two_super_simple() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part_two(&input), 1)
    }

    #[test]
    fn test_part_two_simple() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part_two(&input), 4)
    }

    #[test]
    fn test_part_two_medium() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(part_two(&input), 8)
    }

    #[test]
    fn test_part_two() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part_two(&input), 10)
    }
}
