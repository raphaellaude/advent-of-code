use std::usize;

use petgraph::{graph::Graph, stable_graph::NodeIndex};

pub fn part_one(input: &str) -> usize {
    let (starting_pos, graph) = parse_input(input);

    0
}

pub fn part_two(input: &str) -> usize {
    todo!()
}

#[derive(Debug)]
struct Node {
    loc: usize,
    kind: char,
}

fn parse_input(input: &str) -> (usize, Graph<Node, u32>) {
    let mut graph: Graph<Node, u32> = Graph::new();

    let line_len = input.lines().next().unwrap().len();
    dbg!(line_len);

    let mut starting_position: usize = 0;

    input.lines().enumerate().for_each(|(line_no, l)| {
        l.chars().enumerate().for_each(|(char_no, kind)| {
            let loc = line_len * line_no + char_no;
            let node = Node { loc, kind };
            let idx = graph.add_node(node);

            if kind == 'S' {
                starting_position = idx.index();
            }

            add_edge(loc, kind, line_len, &mut graph);
        })
    });

    if starting_position == 0 {
        panic!("Probably couln't determine starting position!");
    }

    (starting_position, graph)
}

fn add_edge(loc: usize, kind: char, line_len: usize, graph: &mut Graph<Node, u32>) {
    let u = NodeIndex::from(loc as u32);

    match kind {
        'J' => {
            if loc >= line_len {
                graph.add_edge(u, NodeIndex::from((loc - line_len) as u32), 1);
            }
            if loc > 0 {
                graph.add_edge(u, NodeIndex::from((loc - 1) as u32), 1);
            }
        }
        '|' => {
            if loc >= line_len {
                graph.add_edge(u, NodeIndex::from((loc - line_len) as u32), 1);
            }
        }
        'L' => {
            if loc >= line_len {
                graph.add_edge(u, NodeIndex::from((loc - line_len) as u32), 1);
            }
        }
        '-' => {
            if loc > 0 {
                graph.add_edge(u, NodeIndex::from((loc - 1) as u32), 1);
            }
        }
        '.' => (),
        'F' => (),
        'S' => (),
        _ => (),
    }
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
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(part_one(&input), 0)
    }
}
