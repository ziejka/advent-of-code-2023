use rayon::prelude::*;
use std::collections::HashMap;

fn walk(
    instructions_iter: &Vec<usize>,
    codes: &HashMap<Box<str>, [Box<str>; 2]>,
    start_key: &Box<str>,
    num_steps: &mut usize,
) -> u64 {
    let mut key = start_key;
    let has_won = instructions_iter.iter().any(|idx| {
        *num_steps += 1;
        let code = codes.get(key).unwrap();
        key = code.get(*idx).unwrap();

        if key.ends_with('Z') {
            return true;
        }
        return false;
    });

    if has_won {
        return *num_steps as u64;
    }

    walk(instructions_iter, codes, key, num_steps)
}

fn process(s: String) {
    let mut codes: HashMap<Box<str>, [Box<str>; 2]> = HashMap::new();

    let s_without_parentheses = s.replace("(", "").replace(")", "");
    let mut lines = s_without_parentheses.lines();

    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            'L' => Some(0),
            'R' => Some(1),
            _ => None,
        })
        .collect::<Vec<usize>>();

    lines.skip(1).for_each(|line| {
        let mut parts = line.split("=");
        let key = parts.next().unwrap().trim().to_string().into_boxed_str();

        let values_iter = parts.next().unwrap().split(",").collect::<Vec<&str>>();

        codes.insert(
            key,
            [
                values_iter[0].trim().to_string().into_boxed_str(),
                values_iter[1].trim().to_string().into_boxed_str(),
            ],
        );
    });

    let start_keys = codes
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let steps = start_keys
        .into_par_iter()
        .map(|start_key| walk(&instructions, &codes, start_key, &mut 0))
        .collect::<Vec<u64>>();

    println!("min steps: {:?}", steps);
    println!("result : {:?}", steps.lowest_common_multiple());
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        greatest_common_divisor(b, a % b)
    }
}

fn lowest_common_multiple(a: u64, b: u64) -> u64 {
    a * b / greatest_common_divisor(a, b)
}

trait LowestCommonMultiple {
    fn lowest_common_multiple(&self) -> u64;
}

impl LowestCommonMultiple for Vec<u64> {
    fn lowest_common_multiple(&self) -> u64 {
        self.into_iter()
            .fold(1, |a, b| lowest_common_multiple(a, *b))
    }
}

fn main() {
    let start_time = std::time::Instant::now();
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);

    println!("Finished in {:?}", start_time.elapsed());
}
