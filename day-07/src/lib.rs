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

lazy_static! {
    pub static ref FANCY_RANKS: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('A', 0);
        m.insert('K', 1);
        m.insert('Q', 2);
        m.insert('J', 13);
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
            hand_type(cards, false),
        );

        Ok(hand)
    }
}

fn hand_type(cards: &str, is_fancy: bool) -> usize {
    let mut m = HashMap::new();

    for c in cards.chars() {
        if m.contains_key(&c) {
            m.insert(c, m[&c] + 1);
        } else {
            m.insert(c, 1);
        }
    }

    if is_fancy && m.contains_key(&'J') {
        let n_js = m[&'J'];

        if n_js == 5 {
            return 6;
        }

        m.remove(&'J');

        let max_count = m.values().max();
        let best_card =
            m.iter()
                .filter(|(_, v)| *v == max_count.unwrap())
                .fold('J', |acc, (k, _)| {
                    if FANCY_RANKS.get(&k) < FANCY_RANKS.get(&acc) {
                        return *k;
                    }
                    acc
                });
        if best_card == 'J' {
            panic!("Could not determine best card! {:?} {:?}", cards, &m);
        }

        m.insert(best_card, m[&best_card] + n_js);
    }

    // dbg!(cards, &m, score_counts(&m));

    score_counts(&m)
}

fn score_counts(m: &HashMap<char, u8>) -> usize {
    let l = m.len();

    if l == 1 {
        return 6;
    } else if l == 2 {
        if m.values().any(|v| *v == 4) {
            return 5;
        }
        return 4;
    } else if l == 3 {
        if m.values().any(|v| *v == 3) {
            return 3;
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

fn score_hands<'a>(hands: Vec<Hand<'a>>) -> usize {
    hands
        .iter()
        .enumerate()
        .map(|(rank, card)| (rank + 1) * card.bid)
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| Hand::from_string(&line).unwrap())
        .collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    score_hands(hands)
}

#[derive(Eq, PartialEq, Debug)]
struct FancyHand<'a> {
    cards: &'a str,
    bid: usize,
    t: usize,
}

impl<'a> FancyHand<'a> {
    fn new(cards: &'a str, bid: usize, t: usize) -> Self {
        Self { cards, bid, t }
    }

    fn from_string(s: &'a str) -> Result<Self, String> {
        let mut parts = s.split(" ");

        let cards = parts.next().unwrap();
        let hand = FancyHand::new(
            cards,
            parts.next().unwrap().parse::<usize>().unwrap(),
            hand_type(cards, true),
        );

        Ok(hand)
    }
}

impl<'a> PartialOrd for FancyHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for FancyHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t == other.t {
            let iter = self.cards.chars().zip(other.cards.chars());
            for (a, b) in iter {
                let (a_rank, b_rank) = (FANCY_RANKS.get(&a).unwrap(), FANCY_RANKS.get(&b).unwrap());
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

pub fn part_two(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| FancyHand::from_string(&line).unwrap())
        .collect();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(rank, card)| (rank + 1) * card.bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_one(input), 6440);
    }

    #[test]
    fn test_part_two() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_two(input), 5905);
    }

    #[test]
    fn test_fancy_hand_type() {
        assert_eq!(hand_type("3J4KT", true), 1)
    }

    #[test]
    fn test_fancy_hand_type2() {
        assert_eq!(hand_type("Q2KJJ", true), 3);
    }

    #[test]
    fn test_better_input() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
        assert_eq!(part_two(input), 6839);
    }
}
