use std::{
    collections::HashMap,
    str::{FromStr, Lines},
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

    let mut transformers = HashMap::<String, Vec<SourceToDestination>>::new();
    let mut key = "".to_string();
    parse_lines(&mut lines_iter, &mut transformers, &mut key);

    let result = seeds
        .iter()
        .map(|seed_number| {
            let mut source_number = seed_number.clone();

            MAP_ORDER.iter().for_each(|key| {
                let transformations = transformers.get(&key.to_string()).unwrap();

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
            });
            source_number
        })
        .min();

    println!("asd {:#?}", result);
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
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_string();

    process(_input)
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
