#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64,
}

pub fn part_one(input: &str) -> u64 {
    let (times, distances) = parse_seqs(input);

    let races: Vec<_> = std::iter::zip(times, distances)
        .into_iter()
        .map(|(time, distance)| Race { time, distance })
        .collect();

    races
        .iter()
        .map(|race| num_ways_to_beat(race, race.distance))
        .product()
}

pub fn part_two(input: &str) -> u64 {
    let (times, distances) = parse_seqs(input);

    let time = concat_nums(times);
    let distance = concat_nums(distances);

    let races: Vec<_> = vec![Race { time, distance }];

    races
        .iter()
        .map(|race| num_ways_to_beat(race, race.distance))
        .product()
}

fn concat_nums(nums: Vec<u64>) -> u64 {
    let mut result = String::new();

    nums.iter()
        .for_each(|t| result += &(t.to_string().to_owned()));

    result.parse::<u64>().unwrap()
}

fn parse_seqs(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut iter = input.lines();
    let times = parse_line("Time:", iter.next().unwrap());
    let distances = parse_line("Distance:", iter.next().unwrap());

    (times, distances)
}

fn parse_line(start: &str, line: &str) -> Vec<u64> {
    line.strip_prefix(start)
        .unwrap()
        .split(" ")
        .filter(|c| c.len() > 0)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn num_ways_to_beat(race: &Race, time_to_beat: u64) -> u64 {
    let iter = (0..race.time).filter(|elapsed| (elapsed * (race.time - elapsed)) > time_to_beat);
    iter.clone().max().unwrap() - iter.min().unwrap() + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_one(&input), 288);
    }

    #[test]
    fn test_part_two() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_two(&input), 71503);
    }
}
