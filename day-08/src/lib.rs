use std::collections::BTreeMap;

#[derive(Debug)]
struct Edges<'a> {
    left: &'a str,
    right: &'a str,
}

pub fn part_one(input: &str) -> usize {
    let (instructions, graph) = parse_input(input);

    dbg!(instructions);
    dbg!(graph);

    let mut result = 0;

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
}
