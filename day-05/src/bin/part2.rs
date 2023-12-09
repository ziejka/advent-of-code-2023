use indicatif::ProgressBar;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    str::{FromStr, Lines},
    time::Instant,
    vec,
};

#[derive(Debug, PartialEq)]
struct SourceToDestination {
    source_start: u64,
    destination_start: u64,
    size: u64,
}

type Transformations = HashMap<String, Vec<SourceToDestination>>;

impl FromStr for SourceToDestination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        return Ok(Self {
            destination_start: numbers.get(0).unwrap().clone(),
            source_start: numbers.get(1).unwrap().clone(),
            size: numbers.get(2).unwrap().clone(),
        });
    }
}

const MAP_ORDER: [&str; 7] = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location",
];

fn process(str: String) {
    let mut lines_iter = str.lines();
    let seeds = lines_iter
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let seeds_tuple = seeds
        .chunks(2)
        .filter_map(|chunk| {
            if chunk.len() == 2 {
                Some((chunk[0], chunk[1]))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut transformers = HashMap::<String, Vec<SourceToDestination>>::new();
    let mut key = "".to_string();
    parse_lines(&mut lines_iter, &mut transformers, &mut key);

    let total_seeds = seeds_tuple
        .iter()
        .map(|(_, end_number)| end_number)
        .sum::<u64>();
    let pb = ProgressBar::new(total_seeds);

    let total_min_location = seeds_tuple
        .into_par_iter()
        .map(|(start_number, end_number)| {
            let mut min_location = 0;

            let mut seed_number = start_number;
            while seed_number < start_number + end_number {
                let mut source_number: u64 = seed_number;

                for key in &MAP_ORDER {
                    let transformations = transformers.get(*key).unwrap();

                    for transformation in transformations {
                        if source_number >= transformation.source_start
                            && source_number < (transformation.source_start + transformation.size)
                        {
                            let diff = source_number - transformation.source_start;
                            let destination_number = transformation.destination_start + diff;
                            source_number = destination_number.clone();

                            break;
                        }
                    }
                }

                if min_location == 0 || source_number < min_location {
                    min_location = source_number;
                }
                seed_number += 1;
                pb.inc(1)
            }
            min_location
        })
        .min()
        .unwrap_or(0);
    println!("min_location: {}", total_min_location);
}

fn parse_lines(lines_iter: &mut Lines<'_>, transformers: &mut Transformations, key: &mut String) {
    let line = lines_iter.next();

    match line {
        Some(value) => {
            parse_line(value, transformers, key);
            parse_lines(lines_iter, transformers, key);
        }
        None => {}
    }
}

fn parse_line(line: &str, transformers: &mut Transformations, key: &mut String) {
    match line {
        line if line.contains("map:") => {
            if let Some(new_key) = line.split_whitespace().next() {
                *key = new_key.to_string();
                transformers.insert(key.to_string(), vec![]);
            }
        }
        "" => {}
        _ => {
            let numbers = line
                .split_whitespace()
                .filter_map(|num_str| num_str.parse::<u64>().ok())
                .collect::<Vec<u64>>();

            let source_to_destination = SourceToDestination {
                destination_start: numbers.get(0).unwrap().clone(),
                source_start: numbers.get(1).unwrap().clone(),
                size: numbers.get(2).unwrap().clone(),
            };

            if let Some(codes) = transformers.get_mut(key) {
                codes.push(source_to_destination);
            };
        }
    };
}

fn main() {
    let start = Instant::now();

    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    process(_input);

    let duration = start.elapsed();

    println!("Total time elapsed is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_new_key() {
        let mut transformers: Transformations = HashMap::new();
        let mut key = "".to_string();
        parse_line("seed-to-soil map:", &mut transformers, &mut key);
        assert_eq!(transformers.get("seed-to-soil"), Some(vec![]).as_ref());
    }

    #[test]
    fn test_existing_key() {
        let mut transformers: Transformations = HashMap::new();
        let mut key = "seed-to-soil".to_string();
        parse_line("seed-to-soil map:", &mut transformers, &mut key);
        parse_line("10 20 5", &mut transformers, &mut key);
        assert_eq!(
            transformers.get(&key),
            Some(vec![SourceToDestination {
                destination_start: 10,
                source_start: 20,
                size: 5
            }])
            .as_ref()
        );
    }
}
