use std::collections::HashMap;

pub fn part_one(input: &str) -> usize {
    let parsed = parse(input);
    parsed.iter().map(|lens| lens.hash as usize).sum()
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    hash: u8,
    predicate_hash: u8,
    predicate: &'a str,
    is_remove: bool,
    focal_length: Option<u8>,
}

fn parse(input: &str) -> Vec<Lens> {
    input
        .split(',')
        .into_iter()
        .map(|full_name| {
            let hash = hash_lens(&full_name);

            if full_name.contains('-') {
                let mut iter = full_name.split('-');
                let predicate = iter.next().unwrap();

                return Lens {
                    hash,
                    predicate_hash: hash_lens(predicate),
                    predicate,
                    is_remove: true,
                    focal_length: None,
                };
            }
            let mut iter = full_name.split('=');
            let predicate = iter.next().unwrap();
            let focal_length = iter.last();

            Lens {
                hash,
                predicate_hash: hash_lens(predicate),
                predicate,
                is_remove: false,
                focal_length: Some(focal_length.unwrap().parse::<u8>().unwrap()),
            }
        })
        .collect()
}

fn hash_lens(word: &str) -> u8 {
    word.chars().fold(0, |acc, c| {
        (((acc as usize + ((c as u8) as usize)) * 17) % 256) as u8
    })
}

pub fn part_two(input: &str) -> usize {
    let parsed = parse(input);

    let mut m: HashMap<u8, Vec<Lens>> = HashMap::new();

    for s in parsed.iter() {
        if s.is_remove {
            let contents = m.get(&s.predicate_hash);

            if contents.is_some() {
                let mut contents: Vec<Lens> = contents.unwrap().to_vec();
                let index = contents
                    .iter()
                    .position(|lens| lens.predicate == s.predicate);
                if index.is_some() {
                    let index = index.unwrap();
                    contents.remove(index);
                }

                if contents.len() > 0 {
                    m.insert(s.predicate_hash, contents);
                } else {
                    m.remove(&s.predicate_hash);
                }
            }
        } else {
            let contents = m.get(&s.predicate_hash);

            if contents.is_none() {
                m.insert(s.predicate_hash, vec![*s]);
            } else {
                let mut contents = contents.unwrap().to_vec();
                if contents.iter().any(|lens| lens.predicate == s.predicate) {
                    let index = contents
                        .iter()
                        .position(|lens| lens.predicate == s.predicate);
                    if index.is_some() {
                        let index = index.unwrap();
                        contents.remove(index);
                        contents.insert(index, *s);
                    }
                } else {
                    contents.push(*s);
                }
                m.insert(s.predicate_hash, contents);
            }
        }
    }

    m.iter()
        .map(|(h, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(i, lens)| (*h as usize + 1) * (i + 1) * lens.focal_length.unwrap() as usize)
                .sum::<usize>()
        })
        .sum()
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part_two(&input), 145)
    }
}
