use std::str::FromStr;

use anyhow::Ok;

#[derive(Debug, PartialEq)]
struct Game {
    winning_numbers: Vec<i32>,
    drawn_numbers: Vec<i32>,
}

impl Game {
    fn count_won_numbers(&self) -> usize {
        return self
            .drawn_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(&num))
            .count();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning_numbers_str, drawn_numbers_str) = s
            .split_once(":")
            .ok_or(anyhow::anyhow!("Invalid format"))?
            .1
            .split_once("|")
            .ok_or(anyhow::anyhow!("Invalid"))?;

        let winning_numbers = winning_numbers_str
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>();

        let drawn_numbers = drawn_numbers_str
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect::<Vec<_>>();

        return Ok(Game {
            winning_numbers,
            drawn_numbers,
        });
    }
}

fn process(_str: String) -> u32 {
    let r = _str
        .lines()
        .flat_map(|s| s.parse::<Game>())
        .map(|g| g.count_won_numbers())
        .filter(|n| n > &0)
        .map(|n| {
            if n == 1 {
                return 1;
            }
            return 2u32.pow((n - 1) as u32);
        })
        .sum();

    return r;
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");

    let _test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "
    .to_string();

    let result = process(_input);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let game_str = "Card 2: 13 32 20  | 61 30 68";
        let game: Game = game_str.parse().unwrap();

        assert_eq!(
            game,
            Game {
                winning_numbers: vec![13, 32, 20],
                drawn_numbers: vec![61, 30, 68],
            }
        )
    }

    #[test]
    fn test_count_winning_numbers() {
        let game = Game {
            drawn_numbers: vec![1, 2, 3],
            winning_numbers: vec![8, 1, 3],
        };

        assert_eq!(game.count_won_numbers(), 2)
    }
}
