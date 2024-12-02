use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Walker {
    loc: usize,
    dir: Direction,
}

impl Walker {
    fn walk(&mut self, dir: Direction, line_len: &usize) {
        match dir {
            Direction::Left => self.loc -= 1,
            Direction::Right => self.loc += 1,
            Direction::Up => self.loc -= line_len,
            Direction::Down => self.loc += line_len,
        }
        self.dir = dir;
    }
}

pub fn part_one(input: &str) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let cave = parse(input, &line_len);

    count_energized_tiles(0, Direction::Right, &cave, &line_len)
}

fn count_energized_tiles(
    start_loc: usize,
    start_dir: Direction,
    cave: &HashMap<usize, char>,
    line_len: &usize,
) -> usize {
    let n_spots = cave.keys().max().unwrap();

    let start = Walker {
        loc: start_loc,
        dir: start_dir,
    };
    let mut walkers: Vec<Walker> = vec![start];
    let mut visited = HashSet::new();
    visited.insert(start);

    while !walkers.is_empty() {
        let mut new_walkers = Vec::new();

        for walker in walkers.iter() {
            if let Some(next_steps) = next_step(walker, &cave) {
                let mut new_walker = *walker;

                for &dir in &next_steps {
                    if valid_step(&new_walker, &dir, &line_len, &n_spots) {
                        new_walker.walk(dir, &line_len);
                        if !visited.contains(&new_walker) {
                            visited.insert(new_walker);
                            new_walkers.push(new_walker);
                        }
                    }
                }
            }
        }

        walkers.clear();

        walkers.extend(new_walkers);
    }
    visited
        .into_iter()
        .map(|w| w.loc)
        .collect::<HashSet<_>>()
        .into_iter()
        .count()
}

fn parse(input: &str, line_len: &usize) -> HashMap<usize, char> {
    let mut cave: HashMap<usize, char> = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            cave.insert(y * line_len + x, c);
        })
    });

    cave
}

fn next_step(walker: &Walker, cave: &HashMap<usize, char>) -> Option<Vec<Direction>> {
    match cave.get(&walker.loc) {
        Some('.') => Some(vec![walker.dir]),
        Some('\\') => match walker.dir {
            Direction::Left => Some(vec![Direction::Up]),
            Direction::Right => Some(vec![Direction::Down]),
            Direction::Up => Some(vec![Direction::Left]),
            Direction::Down => Some(vec![Direction::Right]),
        },
        Some('/') => match walker.dir {
            Direction::Left => Some(vec![Direction::Down]),
            Direction::Right => Some(vec![Direction::Up]),
            Direction::Up => Some(vec![Direction::Right]),
            Direction::Down => Some(vec![Direction::Left]),
        },
        Some('|') => match walker.dir {
            Direction::Left => Some(vec![Direction::Up, Direction::Down]),
            Direction::Right => Some(vec![Direction::Up, Direction::Down]),
            Direction::Up => Some(vec![Direction::Up]),
            Direction::Down => Some(vec![Direction::Down]),
        },
        Some('-') => match walker.dir {
            Direction::Left => Some(vec![Direction::Left]),
            Direction::Right => Some(vec![Direction::Right]),
            Direction::Up => Some(vec![Direction::Left, Direction::Right]),
            Direction::Down => Some(vec![Direction::Left, Direction::Right]),
        },
        _ => None,
    }
}

fn valid_step(walker: &Walker, dir: &Direction, line_len: &usize, n_spots: &usize) -> bool {
    let remainder = walker.loc % line_len;

    (remainder != 0 && dir == &Direction::Left)
        || (remainder + 1 < *line_len && dir == &Direction::Right)
        || (walker.loc >= *line_len && dir == &Direction::Up)
        || (walker.loc + line_len < *n_spots && dir == &Direction::Down)
}

pub fn part_two(input: &str) -> usize {
    let line_len = input.lines().next().unwrap().len();
    let n_lines = input.lines().count();
    let cave = parse(input, &line_len);

    let mut counts: Vec<usize> = Vec::new();

    (0..line_len).for_each(|idx| {
        counts.push(count_energized_tiles(
            idx,
            Direction::Down,
            &cave,
            &line_len,
        ));

        counts.push(count_energized_tiles(
            n_lines * line_len + idx,
            Direction::Up,
            &cave,
            &line_len,
        ));
    });

    (0..n_lines).for_each(|idx| {
        counts.push(count_energized_tiles(
            idx * line_len,
            Direction::Right,
            &cave,
            &line_len,
        ));

        counts.push(count_energized_tiles(
            idx * line_len + line_len - 1,
            Direction::Left,
            &cave,
            &line_len,
        ));
    });

    counts.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(part_one(&input), 46)
    }
}
