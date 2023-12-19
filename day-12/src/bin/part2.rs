use core::num;

use indicatif::ProgressBar;
use rayon::prelude::*;

fn parse_line(s: String) -> (String, Vec<usize>) {
    let mut iterator = s.split(" ");
    let parts = iterator.next().unwrap();

    let lengths: Vec<usize> = iterator
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    (parts.to_string(), lengths)
}

fn calculate_combinations(parts: &mut Vec<char>, lengths: &mut Vec<usize>, result: &mut usize) {
    println!("{:?} {:?}", parts, lengths);
    if parts.is_empty() {
        if lengths.is_empty() {
            *result += 1;
        }
    } else if lengths.is_empty() {
        if !parts.contains(&'#') {
            *result += 1;
        }
    } else {
        if vec!['.', '#'].contains(&parts[0]) {
            parts.remove(0);
            calculate_combinations(parts, lengths, result)
        } else if vec!['#', '?'].contains(&parts[0]) {
            if lengths[0] <= parts.len()
                && !parts.iter().take(lengths[0]).any(|&c| c == '.')
                && (lengths[0] == parts.len() || parts[lengths[0]] != '#')
            {
                parts.remove(lengths[0] + 1);
                lengths.remove(0);
                calculate_combinations(parts, lengths, result)
            }
        }
    }
}

fn count(config: &mut Vec<char>, numbers: &mut Vec<usize>) -> usize {
    println!("{:?} {:?}", config, numbers);
    if config.is_empty() {
        return if numbers.is_empty() { 1 } else { 0 };
    }

    if numbers.is_empty() {
        return if config.contains(&'#') { 0 } else { 1 };
    }

    let mut result = 0;

    if vec!['.', '?'].contains(&config[0]) {
        config.remove(0);
        result += count(config, numbers);
    }

    if !config.is_empty() && vec!['#', '?'].contains(&config[0]) {
        if numbers[0] <= config.len()
            && !config.iter().take(numbers[0]).any(|&c| c == '.')
            && (numbers[0] == config.len() || config[numbers[0]] != '#')
        {
            config.drain(0..numbers[0] + 1);
            numbers.remove(0);
            result += count(config, numbers);
        }
    }

    result
}

fn process(s: String) {
    let strings = s.lines().map(|l| l.to_string()).collect::<Vec<String>>();
    let pb = ProgressBar::new(strings.len() as u64);

    let r: Vec<usize> = strings
        .into_par_iter()
        .map(|l| parse_line(l))
        .map(|(parts, lengths)| {
            let mut results: usize = 0;
            // let big_parts = std::iter::repeat(parts)
            //     .take(5)
            //     .collect::<Vec<String>>()
            //     .join("?");

            // let mut big_length = std::iter::repeat(lengths)
            //     .take(5)
            //     .flatten()
            //     .collect::<Vec<usize>>();

            // let mut temp_parts: Vec<char> = big_parts.chars().collect();

            let mut t: Vec<char> = parts.clone().chars().collect::<Vec<char>>();
            let mut n = lengths.clone();

            let re = count(&mut t, &mut n);

            pb.inc(1);
            re
        })
        .collect();

    println!("{:?}", r);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process("???.### 1,1,3".to_string());
}
