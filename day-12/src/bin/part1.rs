use rayon::prelude::*;
use std::collections::HashSet;

fn parse_line(s: String) -> (String, Vec<u32>) {
    let mut iterator = s.split(" ");
    let parts = iterator.next().unwrap();

    let lengths: Vec<u32> = iterator
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    (parts.to_string(), lengths)
}

fn is_valid(s: &str, lengths: &Vec<u32>) -> bool {
    let parts_lengths = s
        .split('.')
        .filter(|&p| !p.is_empty())
        .map(|p| p.len() as u32)
        .collect::<Vec<u32>>();
    if parts_lengths.len() != lengths.len() {
        return false;
    }
    &parts_lengths == lengths
}

fn generate_combination(s: &mut Vec<char>, start: usize, result: &mut HashSet<String>) {
    if let Some(i) = s[start..].iter().position(|&c| c == '?') {
        let i = start + i;
        s[i] = '#';
        generate_combination(s, i + 1, result);
        s[i] = '.';
        generate_combination(s, i + 1, result);
        s[i] = '?';
    } else {
        let combination = s.iter().collect::<String>();
        result.insert(combination);
    }
}

fn process(s: String) {
    let r: u32 = s
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .into_par_iter()
        .map(|l| parse_line(l))
        .map(|(parts, lengths)| {
            let mut results = HashSet::new();
            let mut temp_parts: Vec<char> = parts.chars().collect();
            generate_combination(&mut temp_parts, 0, &mut results);

            results.iter().filter(|&r| is_valid(r, &lengths)).count() as u32
        })
        .sum();

    println!("{:?}", r);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fn() {
        let parsed_line = parse_line("#.#.### 1,1,3".to_string());
        let parsed_line_fail = parse_line("##.#.### 1,1,3".to_string());

        assert_eq!(parsed_line, ("#.#.###".to_string(), vec![1, 1, 3]));
        assert_eq!(is_valid(&parsed_line.0, &parsed_line.1), true);
        assert_eq!(is_valid(&parsed_line_fail.0, &parsed_line_fail.1), false);
    }

    #[test]
    fn test_combination_generation() {
        let mut s = vec!['?', '?', '?'];
        let mut result = HashSet::new();
        generate_combination(&mut s, 0, &mut result);

        assert_eq!(result.len(), 8);
    }
}
