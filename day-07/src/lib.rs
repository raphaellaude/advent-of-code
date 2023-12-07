use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

lazy_static! {
    pub static ref RANKS: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('A', 0);
        m.insert('K', 1);
        m.insert('Q', 2);
        m.insert('J', 3);
        m.insert('T', 4);
        m.insert('9', 5);
        m.insert('8', 6);
        m.insert('7', 7);
        m.insert('6', 8);
        m.insert('5', 9);
        m.insert('4', 10);
        m.insert('3', 11);
        m.insert('2', 12);
        m
    };
}

#[derive(Eq, PartialEq, Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
    t: usize,
}

impl<'a> Hand<'a> {
    fn new(cards: &'a str, bid: usize, t: usize) -> Self {
        Self { cards, bid, t }
    }

    fn from_string(s: &'a str) -> Result<Self, String> {
        let mut parts = s.split(" ");

        let cards = parts.next().unwrap();
        let hand = Hand::new(
            cards,
            parts.next().unwrap().parse::<usize>().unwrap(),
            hand_type(cards),
        );

        Ok(hand)
    }
}

fn hand_type(cards: &str) -> usize {
    let mut m = HashMap::new();

    for c in cards.chars() {
        if m.contains_key(&c) {
            let tot = m[&c] + 1;
            if tot == 5 {
                return 6;
            } else if tot == 4 {
                return 5;
            }
            m.insert(c, tot);
        } else {
            m.insert(c, 1);
        }
    }

    let l = m.len();
    if l == 2 {
        return 4;
    } else if l == 3 {
        for v in m.values() {
            if *v == 3 {
                return 3;
            }
        }
        return 2;
    } else if l == 4 {
        return 1;
    }

    0
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t == other.t {
            let iter = self.cards.chars().zip(other.cards.chars());
            for (a, b) in iter {
                let (a_rank, b_rank) = (RANKS.get(&a).unwrap(), RANKS.get(&b).unwrap());
                if a_rank < b_rank {
                    return Ordering::Greater;
                } else if a_rank > b_rank {
                    return Ordering::Less;
                }
            }
            panic!(
                "Cards have equal rank! Must be identical {:?} {:?}",
                self.cards, other.cards
            );
        } else if self.t < other.t {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut hands: Vec<_> = input.lines().map(|line| Hand::from_string(&line)).collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // dbg!(&hands);

    hands
        .iter()
        .enumerate()
        .map(|(rank, card)| (rank + 1) * card.as_ref().unwrap().bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_part_one() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_one(input), 6440);
    }
}
