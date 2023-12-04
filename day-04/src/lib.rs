use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryFrom;

pub fn part_one(input: &str) -> u32 {
    let base: u32 = 2;

    input
        .lines()
        .map(|line| {
            let count = intersection_count(parse_line(&line));
            if count <= 1 {
                return base.pow(count) - 1;
            }
            base.pow(count - 1)
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut counts: Vec<u32> = input
        .lines()
        .map(|line| intersection_count(parse_line(&line)))
        .collect();

    let mut idx = 0;

    while idx < 10 {
        let count = usize::try_from(*&counts[idx]).unwrap();

        let to_insert = counts[(idx * 2 + 1)..(idx * 2 + count + 1)]
            .to_owned()
            .clone();
        dbg!(&to_insert);

        for x in 0..(count) {
            let left_key = idx + x + 2;
            // let right_key = idx * 2 + (x - 1) * 2 + 1;
            dbg!(idx, left_key, to_insert[x]);
            counts.insert(left_key, to_insert[x]);
        }

        println!("{:?}", &counts);

        idx += 1;
    }

    counts.len()
}

fn parse_line(line: &str) -> (HashSet<i32>, HashSet<i32>) {
    let sets: Vec<HashSet<i32>> = line
        .split(": ")
        .last()
        .unwrap()
        .split(" | ")
        .map(|s| {
            let mut set = HashSet::new();
            s.split(' ').filter(|s| s.len() > 0).for_each(|i| {
                set.insert(i.trim().parse::<i32>().unwrap());
            });
            set
        })
        .collect();

    let (set1, set2) = sets.iter().next_tuple().unwrap();

    (set1.clone(), set2.clone())
}

fn intersection_count((set1, set2): (HashSet<i32>, HashSet<i32>)) -> u32 {
    set1.iter()
        .filter(|c| set2.contains(c))
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let mut set1 = HashSet::new();
        set1.insert(1);
        set1.insert(2);
        let mut set2 = HashSet::new();
        set2.insert(2);
        set2.insert(3);

        assert_eq!(1, intersection_count((set1, set2)));
    }

    #[test]
    fn test_part_one() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_two(&input), 30);
    }
}
