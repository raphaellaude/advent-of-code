use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let line_len = input.lines().next().unwrap().len();

    let (s_loc, hash_map) = parse_input(input, line_len);
    let mut longest_path: Vec<usize> = Vec::new();

    let first_moves = get_first_moves(s_loc, line_len);
    // dbg!(&first_moves);

    let mut path_lens: Vec<usize> = Vec::new();

    for first_move in first_moves {
        let mut prev_loc = s_loc;
        // println!("Starting at {first_move}");
        let mut current_loc = Some(first_move);

        while current_loc.is_some() {
            let node = current_loc.unwrap();
            let kind = *hash_map.get(&node).unwrap();

            // println!("{node} {prev_loc} {kind}");

            if kind == 'S' {
                // || longest_path.len() > 10
                break;
            }

            current_loc = next_loc(node, prev_loc, kind, line_len);

            prev_loc = node;
            longest_path.push(node);
        }

        path_lens.push(longest_path.len());
        longest_path.clear();
    }

    let distance = *path_lens.iter().max().unwrap() as u32;

    if distance % 2 == 0 {
        println!("Is even!!!");
        return distance / 2;
    }
    distance / 2 + 1
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

pub fn part_two(_input: &str) -> usize {
    todo!()
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
            // println!("AT F!!!! {loc} {prev_loc}");
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
