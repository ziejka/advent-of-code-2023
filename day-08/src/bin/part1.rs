use std::collections::HashMap;

fn walk(
    instructions_iter: &[usize],
    codes: &HashMap<Box<str>, [Box<str>; 2]>,
    start_key: &Box<str>,
    winning_key: &Box<str>,
    num_steps: &mut usize,
) {
    let mut key = start_key;
    let has_won = instructions_iter.iter().any(|idx| {
        *num_steps += 1;
        let code = codes.get(key).unwrap();
        key = code.get(*idx).unwrap();

        if key == winning_key {
            return true;
        }
        return false;
    });

    if has_won {
        println!("Found ZZZ in {} steps", num_steps);
        return;
    }

    walk(instructions_iter, codes, key, winning_key, num_steps)
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

    walk(
        &instructions,
        &codes,
        &"AAA".to_string().into_boxed_str(),
        &"ZZZ".to_string().into_boxed_str(),
        &mut 0,
    );
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);
}
