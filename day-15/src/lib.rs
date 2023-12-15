pub fn part_one(input: &str) -> usize {
    let parsed = parse(input);
    parsed.iter().map(|s| hash(s)).sum()
}

fn parse(input: &str) -> Vec<&str> {
    input.split(',').into_iter().collect()
}

fn hash(word: &str) -> usize {
    word.chars()
        .fold(0, |acc, c| ((acc + ((c as u8) as usize)) * 17) % 256)
}

pub fn part_two(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_one(&input), 1320)
    }

    #[test]
    fn test_part_two() {
        let input = "";
        assert_eq!(part_two(&input), 145)
    }
}
