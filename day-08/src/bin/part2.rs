use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn step<'a>(
    codes: &'a HashMap<String, [String; 2]>,
    key: &String,
    idx: &usize,
    is_done: &mut bool,
) -> &'a String {
    *is_done = false;

    let code = codes.get(key).unwrap();
    let new_key = code.get(*idx).unwrap();

    if new_key.ends_with('Z') {
        *is_done = true;
    }
    return new_key;
}

fn process(s: String) {
    let mut codes: HashMap<String, [String; 2]> = HashMap::new();

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
        let key = parts.next().unwrap().trim().to_string();

        let values_iter = parts.next().unwrap().split(",").collect::<Vec<&str>>();

        codes.insert(
            key,
            [
                values_iter[0].trim().to_string(),
                values_iter[1].trim().to_string(),
            ],
        );
    });

    let mut start_keys: HashSet<&String> = codes.keys().filter(|k| k.ends_with('A')).collect();
    let mut num_steps: usize = 0;

    walk(&mut start_keys, &codes, &mut num_steps, &instructions)
}

fn walk<'a>(
    keys: &mut HashSet<&'a String>,
    codes: &'a HashMap<String, [String; 2]>,
    num_steps: &mut usize,
    instructions: &'a Vec<usize>,
) {
    let mut all_done: Vec<bool> = vec![false];

    while all_done.contains(&false) {
        for idx in instructions.iter() {
            *num_steps += 1;
            all_done = vec![];

            let mut temp_keys: Vec<&String> = Vec::new();

            for start_key in keys.iter() {
                let mut is_done = false;
                let new_key = step(&codes, start_key, idx, &mut is_done);
                temp_keys.push(new_key);
                all_done.push(is_done);
            }

            keys.clear();
            for new_key in temp_keys {
                keys.insert(new_key);
            }

            if !all_done.contains(&false) {
                println!("Found ZZZ in {} steps", num_steps);
                return;
            }
        }
    }
    println!("Found ZZZ in {} steps", num_steps);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test2").expect("file name input");

    process(_input);
}
