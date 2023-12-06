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

    dbg!(&races);

    races
        .iter()
        .map(|race| num_ways_to_beat(race, race.distance))
        .product()
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
        .map(|s| {
            dbg!(s);
            s.parse::<u64>().unwrap()
        })
        .collect::<Vec<u64>>()
}

fn num_ways_to_beat(race: &Race, time_to_beat: u64) -> u64 {
    let iter = (0..race.time).filter(|elapsed| {
        dbg!(elapsed * (race.time - elapsed));
        (elapsed * (race.time - elapsed)) > time_to_beat
    });

    let (max, min) = (iter.clone().max().unwrap(), iter.min().unwrap());
    dbg!(vec![&min, &max]);

    max - min + 1
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
}
