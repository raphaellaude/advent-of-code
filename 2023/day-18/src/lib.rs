use glam::I64Vec2;
use itertools::Itertools;
use std::i64;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug)]
struct Edge {
    direction: Direction,
    length: i64,
}

pub fn part_one(input: &str) -> i64 {
    let edges = parse_pt_one(input);
    let points = collect_points(edges);
    picks_area(points)
}

fn parse_pt_one(input: &str) -> Vec<Edge> {
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
                length: iter.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect()
}

fn collect_points(edges: Vec<Edge>) -> Vec<I64Vec2> {
    let start = I64Vec2 { x: 0, y: 0 };

    let mut points = vec![start];

    edges.iter().enumerate().for_each(|(n_points, edge)| {
        let position = points[n_points];

        match edge.direction {
            Direction::Up => points.push(I64Vec2 {
                x: position.x,
                y: position.y - edge.length,
            }),
            Direction::Down => points.push(I64Vec2 {
                x: position.x,
                y: position.y + edge.length,
            }),
            Direction::Left => points.push(I64Vec2 {
                x: position.x - edge.length,
                y: position.y,
            }),
            Direction::Right => points.push(I64Vec2 {
                x: position.x + edge.length,
                y: position.y,
            }),
        };
    });

    points
}

fn picks_area(points: Vec<I64Vec2>) -> i64 {
    points
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x + (b.x - a.x + b.y - a.y).abs())
        .sum::<i64>()
        / 2
        + 1
}

pub fn part_two(input: &str) -> i64 {
    let edges = parse_pt_two(input);
    let points = collect_points(edges);
    picks_area(points)
}

fn parse_pt_two(input: &str) -> Vec<Edge> {
    input
        .lines()
        .map(|line| {
            let color = line.split(' ').last().unwrap().to_string();
            Edge {
                direction: match color.chars().nth(7).unwrap() {
                    '3' => Direction::Up,
                    '1' => Direction::Down,
                    '0' => Direction::Right,
                    '2' => Direction::Left,
                    _ => panic!("Invalid direction"),
                },
                length: i64::from_str_radix(&color[2..7], 16).unwrap(),
            }
        })
        .collect()
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

    #[test]
    fn test_part_two() {
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
        assert_eq!(part_two(&input), 952_408_144_115)
    }
}
