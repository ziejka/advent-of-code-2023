use std::{collections::HashMap, str::FromStr, time::Instant};

use anyhow::Ok;

#[derive(Debug, PartialEq, Clone)]
struct Game {
    id: usize,
    won_numbers_count: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_str, numbers_str) = s.split_once(":").ok_or(anyhow::anyhow!("Invalid format"))?;

        let game_idx = game_str
            .split_whitespace()
            .last()
            .ok_or(anyhow::anyhow!("Invalid ID"))?
            .parse::<usize>()?;

        let (winning_numbers_str, drawn_numbers_str) = numbers_str
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

        let won_numbers_count = drawn_numbers
            .iter()
            .filter(|num| winning_numbers.contains(num))
            .count();

        return Ok(Game {
            id: game_idx,
            won_numbers_count,
        });
    }
}

fn process(_str: String) -> usize {
    let games: Vec<Game> = _str
        .lines()
        .filter_map(|s| s.parse::<Game>().ok())
        .collect();

    let mut result: HashMap<usize, usize> = games.iter().map(|g| (g.id, 1)).collect();

    for game in games {
        let indexes_to_copy = (game.id + 1)..=(game.id + game.won_numbers_count);
        let number_of_copies = *result.get(&game.id).expect("value");

        for game_index in indexes_to_copy {
            if let Some(value) = result.get_mut(&game_index) {
                *value += number_of_copies;
            }
        }
    }

    return result.values().sum();
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

    let start = Instant::now();
    let result = process(_input);
    println!("{:?}", result);
    let duration = start.elapsed();

    println!("Time elapsed in process() is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_game() {
        let game_str = "Card 2: 13 32 20  | 61 32 68";
        let game: Game = game_str.parse().unwrap();

        assert_eq!(
            game,
            Game {
                id: 2,
                won_numbers_count: 1,
            }
        )
    }
}
