use std::collections::BTreeMap;

#[derive(Debug)]
struct Edges<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_one(input: &str) -> usize {
    let (instructions, mut graph) = parse_input(input);

    // dbg!(instructions);
    // dbg!(&graph);

    let mut result = 0;

    let first_node = &graph.first_entry().unwrap();
    let mut current_node = *first_node.key();

    while current_node != "ZZZ" {
        for next_move in instructions.chars() {
            let edges = &graph.get(current_node).unwrap();
            if next_move == 'L' {
                current_node = edges.left;
            } else {
                current_node = edges.right;
            }
            result += 1;
        }
    }

    result
}

fn parse_input<'a>(input: &'a str) -> (&str, BTreeMap<&str, Edges>) {
    let mut iter = input.lines();
    let instructions = iter.next().unwrap();

    iter.next();

    let mut graph: BTreeMap<&'a str, Edges> = BTreeMap::new();
    iter.for_each(|l| add_node_and_edges(l, &mut graph));

    (instructions, graph)
}

fn add_node_and_edges<'a>(line: &'a str, graph: &mut BTreeMap<&'a str, Edges<'a>>) {
    let mut parts = line.split(" = ");
    let node = parts.next().unwrap();
    let lr: &str = parts.next().unwrap();

    let left: &'a str = &lr[1..=3];
    let right: &'a str = &lr[6..=8];

    graph.insert(node, Edges { left, right });
}

pub fn part_two(input: &str) -> usize {
    let (instructions, graph) = parse_input(input);

    // dbg!(instructions);
    // dbg!(&graph);

    let mut result = 0;

    let mut current_nodes: Vec<&str> = graph
        .keys()
        .filter(|k| k.chars().last().unwrap() == 'A')
        .copied()
        .collect();

    // dbg!(&graph);

    while !current_nodes
        .iter()
        .all(|n| n.chars().last().unwrap() == 'Z')
    {
        for next_move in instructions.chars() {
            current_nodes = current_nodes
                .iter()
                .map(|c| {
                    let e = graph.get(*c).unwrap();
                    if next_move == 'L' {
                        e.left
                    } else {
                        e.right
                    }
                })
                .collect();
            // dbg!(&current_nodes);
            result += 1;
            println!("{result}");
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input), 2)
    }

    #[test]
    fn test_part_one2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part_one(input), 6)
    }

    #[test]
    fn test_part_two1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part_two(input), 6)
    }
}
