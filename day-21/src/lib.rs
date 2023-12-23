use petgraph::{
    graph::{NodeIndex, UnGraph},
    Graph, Undirected,
};
use std::collections::{BTreeMap, HashSet};

pub fn part_one(input: &str, objective: u8) -> usize {
    let (graph, m) = parse_enclosed_garden(input);
    count_plots(objective, graph, m)
}

fn count_plots(
    objective: u8,
    graph: Graph<u32, u32, Undirected>,
    m: BTreeMap<usize, char>,
) -> usize {
    let bleh = m
        .iter()
        .filter(|(_, c)| **c == 'S')
        .map(|(loc, _)| loc.clone())
        .collect::<Vec<usize>>();
    let mut positions: HashSet<usize> = HashSet::from_iter(bleh);

    for _ in 0..objective {
        positions = HashSet::from_iter(
            positions
                .iter()
                .flat_map(|p| {
                    let neighs = graph.neighbors(NodeIndex::new(*p));
                    neighs.map(|n| n.index())
                })
                .collect::<Vec<usize>>(),
        );
    }

    positions.len()
}

fn parse_enclosed_garden(input: &str) -> (Graph<u32, u32, Undirected>, BTreeMap<usize, char>) {
    let line_len = input.lines().next().unwrap().len();
    let mut m = BTreeMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            m.insert(y * line_len + x, c);
        })
    });

    let mut graph = UnGraph::<u32, u32>::new_undirected();
    let mut node_indices = BTreeMap::new();

    for &loc in m.keys() {
        let node_index = graph.add_node(loc as u32);
        node_indices.insert(loc, node_index);
    }

    for (loc, c) in m.iter() {
        if loc == &0 || c == &'#' {
            continue;
        }
        if loc >= &line_len {
            // add up
            if to_add(&m.get(&(loc - line_len)).unwrap()) {
                graph.add_edge(node_indices[&loc], node_indices[&(loc - line_len)], 1);
            }
        }
        if loc % line_len > 0 {
            // add left
            if to_add(&m.get(&(loc - 1)).unwrap()) {
                graph.add_edge(node_indices[&loc], node_indices[&(loc - 1)], 1);
            }
        }
    }

    (graph, m)
}

fn to_add(c: &char) -> bool {
    match c {
        '#' => false,
        '.' => true,
        'S' => true,
        _ => panic!("Couldn't find match!"),
    }
}

// pub fn part_two(input: &str, objective: u8) -> usize {
//     let (graph, m) = parse_infinite_garden(input);
//     todo!()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_one(&input, 6), 16)
    }

    #[test]
    fn test_part_two() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part_two(&input, 50), 1594)
    }
}
