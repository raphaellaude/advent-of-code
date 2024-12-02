use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap, HashMap};
use std::vec;

pub fn part_one(input: &str) -> usize {
    let line_len = input.lines().next().unwrap().len() as i32;
    let n_lines = input.lines().count() as i32;

    let m = parse(input);

    let start = Pos {
        x: (0..line_len).into_iter().fold(
            0,
            |acc, x| {
                if m[&Pos { x, y: 0 }] == '.' {
                    x
                } else {
                    acc
                }
            },
        ),
        y: 0,
    };

    let dest = Pos {
        x: (0..line_len).into_iter().fold(0, |acc, x| {
            if m[&Pos { x, y: n_lines - 1 }] == '.' {
                x
            } else {
                acc
            }
        }),
        y: n_lines - 1,
    };

    let path = longest_path(m, start, dest);

    path.len()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
}

const RIGHT: Pos = Pos { x: 1, y: 0 };
const LEFT: Pos = Pos { x: -1, y: 0 };
const UP: Pos = Pos { x: 0, y: -1 };
const DOWN: Pos = Pos { x: 0, y: 1 };

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: i32,
    pos: Pos,
    prev_loc: Option<Pos>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> BTreeMap<Pos, char> {
    let mut m = BTreeMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            m.insert(
                Pos {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        })
    });

    m
}

fn longest_path(m: BTreeMap<Pos, char>, source: Pos, dest: Pos) -> Vec<Pos> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut heap = BinaryHeap::new();

    let path_chars = [&'>', &'<', &'^', &'v', &'.'];

    for (node, v) in m.iter() {
        if path_chars.contains(&v) {
            dist.insert(*node, i32::MAX);
            prev.insert(*node, None);
        }
    }

    dist.insert(source, -1);
    heap.push(State {
        cost: -1,
        pos: source,
        prev_loc: None,
    });

    while let Some(State {
        cost,
        pos,
        prev_loc,
    }) = heap.pop()
    {
        if cost > dist[&pos] || pos == dest {
            continue;
        }

        for v in neighbors(pos, prev_loc, &m) {
            let next = State {
                cost: cost - 1,
                pos: v,
                prev_loc: Some(pos),
            };

            if next.cost < dist[&v] {
                dist.insert(v, next.cost);
                prev.insert(v, Some(pos));
                heap.push(next);
            }
        }
    }

    let mut path = vec![];
    let mut current = dest;

    while current != source {
        path.push(current);
        current = prev[&current].unwrap();
    }

    path.reverse();

    return path;
}

fn neighbors(pos: Pos, prev_loc: Option<Pos>, m: &BTreeMap<Pos, char>) -> Vec<Pos> {
    let mut neighs = Vec::new();

    if prev_loc.is_none() {
        neighs.push(Pos {
            x: pos.x,
            y: pos.y + 1,
        })
    } else {
        let prev_loc = prev_loc.unwrap();
        let dir = Pos {
            x: pos.x - prev_loc.x,
            y: pos.y - prev_loc.y,
        };

        let straight = Pos {
            x: pos.x + dir.x,
            y: pos.y + dir.y,
        };
        let right = Pos {
            x: pos.x + dir.y,
            y: pos.y + dir.x,
        };
        let left = Pos {
            x: pos.x - dir.y,
            y: pos.y - dir.x,
        };

        for p in vec![straight, right, left] {
            if is_valid_next_step(&p, &prev_loc, &dir, m) {
                neighs.push(p);
            }
        }
    }

    if neighs.len() == 0 {
        dbg!(pos, prev_loc, m[&pos], m[&prev_loc.unwrap()]);
        panic!("No neighbors found!");
    }

    neighs
}

fn is_valid_next_step(pos: &Pos, prev_loc: &Pos, dir: &Pos, m: &BTreeMap<Pos, char>) -> bool {
    let valid_loc = m.contains_key(pos);
    if !valid_loc {
        return false;
    }
    let c = m[pos];
    if c == '#' {
        return false;
    }

    match dir {
        &RIGHT => {
            ['>', '.'].contains(&c)
                || (c == 'v' && pos.y > prev_loc.y)
                || (c == '^' && pos.y < prev_loc.y)
        }
        &LEFT => {
            ['<', '.'].contains(&c)
                || (c == 'v' && pos.y > prev_loc.y)
                || (c == '^' && pos.y < prev_loc.y)
        }
        &UP => {
            ['^', '.'].contains(&c)
                || (c == '>' && pos.x > prev_loc.x)
                || (c == '<' && pos.x < prev_loc.x)
        }
        &DOWN => {
            ['v', '.'].contains(&c)
                || (c == '>' && pos.x > prev_loc.x)
                || (c == '<' && pos.x < prev_loc.x)
        }
        _ => false,
    }
}

pub fn part_two(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part_one(&input), 94)
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_two(&input), 0)
    // }
}
