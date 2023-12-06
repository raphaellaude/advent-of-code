use std::collections::HashMap;

pub fn part_one(input: &str) -> u64 {
    let mut iter = input.split("\n\n");
    let initial_seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    println!("{:?}", initial_seeds);

    let sets: Vec<HashMap<u64, u64>> = iter
        .map(|section| {
            let mut lines = section.split(":\n");
            lines.next();
            map_from_ranges(lines.next().unwrap())
        })
        .collect();

    println!("Sets assembled");

    initial_seeds
        .iter()
        .map(|seed| location_from_seed(*seed, &sets))
        .min()
        .expect("Was not able to determine min seed value.")
}

fn location_from_seed(seed: u64, sets: &Vec<HashMap<u64, u64>>) -> u64 {
    let mut result: u64 = seed;

    for set in sets {
        result = map_value(result, set);
    }

    result
}

fn map_from_ranges(input: &str) -> HashMap<u64, u64> {
    let mut set: HashMap<u64, u64> = HashMap::new();

    input.lines().for_each(|line| {
        let mut iter = line.split(' ');
        let source = iter.next().unwrap().parse::<u64>().unwrap();
        let dest = iter.next().unwrap().parse::<u64>().unwrap();
        let n_steps = iter.next().unwrap().parse::<u64>().unwrap();

        for idx in 0..n_steps {
            set.insert(dest + idx, source + idx);
        }
    });

    set
}

fn map_value(value: u64, set: &HashMap<u64, u64>) -> u64 {
    match set.get(&value) {
        Some(result) => *result,
        _ => value,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(part_one(input), 35);
    }
}
