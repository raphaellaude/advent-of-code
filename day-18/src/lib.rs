use glam::i32::IVec2;
use itertools::Itertools;

pub fn part_one(input: &str) -> i32 {
    let edges = parse(input);
    let start = IVec2 { x: 0, y: 0 };

    let mut points = vec![start];

    edges.iter().enumerate().for_each(|(n_points, edge)| {
        let position = points[n_points];

        match edge.direction {
            Direction::Up => points.push(IVec2 {
                x: position.x,
                y: position.y - edge.length,
            }),
            Direction::Down => points.push(IVec2 {
                x: position.x,
                y: position.y + edge.length,
            }),
            Direction::Left => points.push(IVec2 {
                x: position.x - edge.length,
                y: position.y,
            }),
            Direction::Right => points.push(IVec2 {
                x: position.x + edge.length,
                y: position.y,
            }),
        };
    });

    points
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x + (b.x - a.x + b.y - a.y).abs())
        .sum::<i32>()
        / 2
        + 1
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Edge<'a> {
    direction: Direction,
    length: i32,
    _color: &'a str,
}

fn parse(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            Edge {
                direction: match iter.next().unwrap() {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    _ => panic!("Invalid direction"),
                },
                length: iter.next().unwrap().parse::<i32>().unwrap(),
                _color: iter.next().unwrap(),
            }
        })
        .collect()
}

pub fn part_two(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part_one(&input), 62)
    }

    // #[test]
    // fn test_part_two() {
    //     let input = "";
    //     assert_eq!(part_two(&input), 0)
    // }
}
