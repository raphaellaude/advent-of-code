use indicatif::ProgressIterator;
use itertools::Itertools;
use std::str;

pub fn part_one(input: &str) -> usize {
    let mut iter = input.split("\n\n");
    let initial_seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("{:?}", initial_seeds);

    let sets: Vec<Vec<(usize, usize, usize)>> = get_sets(iter);

    initial_seeds
        .iter()
        .map(|seed| location_from_seed(*seed, &sets))
        .min()
        .expect("Was not able to determine min seed value.")
}

pub fn part_two(input: &str) -> usize {
    let mut iter = input.split("\n\n");

    let all_seeds = iter
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let sets: Vec<Vec<(usize, usize, usize)>> = get_sets(iter);

    let mut min_val = 18446744073709551615;

    for (start, end) in (0..all_seeds.len()).tuples() {
        println!(
            "Starting next batch {:?}",
            vec![&all_seeds[start], &all_seeds[end]]
        );

        for seed in (all_seeds[start]..all_seeds[start] + all_seeds[end]).progress() {
            let location = location_from_seed(seed, &sets);

            if location < min_val {
                println!("New min val {location} for {seed}!");
                min_val = location;
            }
        }
    }

    min_val
}

fn get_sets(iter: str::Split<'_, &str>) -> Vec<Vec<(usize, usize, usize)>> {
    iter.map(|section| {
        let mut lines = section.split(":\n");
        lines.next();
        map_from_ranges(lines.next().unwrap())
    })
    .collect()
}

fn map_from_ranges(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let source = iter.next().unwrap().parse::<usize>().unwrap();
            let dest = iter.next().unwrap().parse::<usize>().unwrap();
            let n_steps = iter.next().unwrap().parse::<usize>().unwrap();

            (source, dest, n_steps)
        })
        .collect()
}

fn location_from_seed(seed: usize, sets: &Vec<Vec<(usize, usize, usize)>>) -> usize {
    let mut result: usize = seed;

    for set in sets {
        result = map_value(result, set);
    }

    result
}

fn map_value(value: usize, set: &Vec<(usize, usize, usize)>) -> usize {
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
