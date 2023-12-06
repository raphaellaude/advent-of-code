use itertools::Itertools;
use std::str;

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

    let sets: Vec<Vec<(u64, u64, u64)>> = get_sets(iter);

    initial_seeds
        .iter()
        .map(|seed| location_from_seed(*seed, &sets))
        .min()
        .expect("Was not able to determine min seed value.")
}

pub fn part_two(input: &str) -> u64 {
    let mut iter = input.split("\n\n");

    let all_seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let sets: Vec<Vec<(u64, u64, u64)>> = get_sets(iter);

    let mut min_val = 18446744073709551615;

    for (start, end) in (0..all_seeds.len()).tuples() {
        dbg!(vec![&all_seeds[start], &all_seeds[end]]);

        for seed in all_seeds[start]..all_seeds[start] + all_seeds[end] {
            let location = location_from_seed(seed, &sets);

            if location < min_val {
                println!("New min val {min_val} for {seed}!");
                min_val = location;
            }
        }
    }

    min_val
}

fn get_sets(iter: str::Split<'_, &str>) -> Vec<Vec<(u64, u64, u64)>> {
    iter.map(|section| {
        let mut lines = section.split(":\n");
        lines.next();
        map_from_ranges(lines.next().unwrap())
    })
    .collect()
}

fn location_from_seed(seed: u64, sets: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    let mut result: u64 = seed;

    for set in sets {
        result = map_value(result, set);
    }

    result
}

fn map_from_ranges(input: &str) -> Vec<(u64, u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let source = iter.next().unwrap().parse::<u64>().unwrap();
            let dest = iter.next().unwrap().parse::<u64>().unwrap();
            let n_steps = iter.next().unwrap().parse::<u64>().unwrap();

            (source, dest, n_steps)
        })
        .collect()
}

fn map_value(value: u64, set: &Vec<(u64, u64, u64)>) -> u64 {
    for (source, dest, n_steps) in set {
        if &value >= dest && value < dest + n_steps {
            return value - dest + source;
        }
    }
    value
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
