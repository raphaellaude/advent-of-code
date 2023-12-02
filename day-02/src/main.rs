use std::cmp::Ordering;
use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn from_string(s: &str) -> Result<Self, String> {
        // example input: 6 green, 3 blue
        let mut parts = s.split(", ");
        let mut rgb = RGB::new(0, 0, 0);

        while let Some(part) = parts.next() {
            let mut part = part.split(' ');
            let count = part.next().unwrap().parse::<u8>().unwrap();
            let color = part.next().unwrap();

            match color {
                "red" => rgb.red = count,
                "green" => rgb.green = count,
                "blue" => rgb.blue = count,
                _ => return Err(format!("Unknown color: {}", color)),
            }
        }

        Ok(rgb)
    }

    fn power(&self) -> u32 {
        let red = if self.red > 0 { self.red as u32 } else { 1 };
        let green = if self.green > 0 { self.green as u32 } else { 1 };
        let blue = if self.blue > 0 { self.blue as u32 } else { 1 };

        red * green * blue
    }
}

impl PartialOrd for RGB {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RGB {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.red == other.red && self.green == other.green && self.blue == other.blue {
            return Ordering::Equal;
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

const AVAILABLE_COLORS: RGB = RGB {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let result = part_one(&input);
    println!("Total games possible {:?}", result);

    let result = part_two(&input);
    println!("Total power of games {:?}", result);
}

fn part_one(input: &String) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if valid_game(line) {
                return i + 1;
            }
            0
        })
        .sum()
}

fn parse_game(game: &str) -> Vec<RGB> {
    let game = game.split(": ").last().unwrap();

    game.split(';')
        .map(|s| RGB::from_string(s.trim()).unwrap())
        .collect()
}

fn valid_game(game: &str) -> bool {
    let game = parse_game(game);
    game.iter().all(|rgb| rgb <= &AVAILABLE_COLORS)
}

fn part_two(input: &String) -> u32 {
    let games = input
        .lines()
        .map(|line| parse_game(line))
        .collect::<Vec<Vec<RGB>>>();

    let mut total_power = 0;

    for game in games {
        let max_color = max_color(game);
        total_power += max_color.power();
    }

    total_power
}

fn max_color(game: Vec<RGB>) -> RGB {
    let mut out_color = RGB::new(0, 0, 0);

    for rgb in game {
        if rgb.red > out_color.red {
            out_color.red = rgb.red;
        }

        if rgb.green > out_color.green {
            out_color.green = rgb.green;
        }

        if rgb.blue > out_color.blue {
            out_color.blue = rgb.blue;
        }
    }
    // dbg!(&out_color);

    out_color
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        false
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", true)]
    fn test_valid_game(#[case] game: &str, #[case] expected: bool) {
        assert_eq!(valid_game(game), expected);
    }

    #[test]
    fn test_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_one(&input.to_string()), 8)
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        1560
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        630
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_max_color(#[case] game: &str, #[case] expected: u32) {
        let game = parse_game(game);
        assert_eq!(max_color(game).power(), expected);
    }
}
