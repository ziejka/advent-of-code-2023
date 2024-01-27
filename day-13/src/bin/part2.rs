fn rotate_pattern(pattern: &[&str]) -> Vec<String> {
    let mut new_vec: Vec<String> = vec!["".to_string(); pattern[0].len()];

    pattern.iter().for_each(|str| {
        str.chars().enumerate().for_each(|(idx, ch)| {
            new_vec[idx].push(ch);
        })
    });
    new_vec
}

fn is_equal_with_smudge(l1: &str, l2: &str) -> bool {
    let mut distance = 0;
    for (idx, ch) in l1.chars().enumerate() {
        if ch != l2.chars().nth(idx).unwrap() {
            distance += 1;

            if distance > 1 {
                return false;
            }
        }
    }
    println!("{:?} {:?}", l1, l2);
    return true;
}

fn is_reflection_from_idx(pattern: &[&str], start: usize) -> bool {
    let mut i = start;
    let mut j = start + 1;

    while j < pattern.len() {
        if !is_equal_with_smudge(pattern[i], pattern[j]) {
            return false;
        }
        if i > 0 {
            i -= 1;
        } else {
            return true;
        }
        j += 1;
    }
    return true;
}

fn get_reflection_point(pattern: &[&str]) -> Option<usize> {
    for ((idx, l1), l2) in pattern
        .iter()
        .enumerate()
        .take(pattern.len() - 1)
        .zip(pattern.iter().skip(1))
    {
        if l1 == l2 && is_reflection_from_idx(pattern, idx) {
            return Some(idx + 1);
        }
    }

    return None;
}

fn process(s: String) {
    let mut patterns: Vec<Vec<&str>> = Vec::new();
    let mut pattern = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            patterns.push(pattern.clone());
            pattern.clear();
            continue;
        }
        pattern.push(line);
    }
    patterns.push(pattern.clone());

    let r: usize = patterns
        .iter()
        .map(|pattern| {
            if let Some(row_idx) = get_reflection_point(pattern) {
                return row_idx * 100;
            }

            let rotated = rotate_pattern(pattern);
            let rotated_str: Vec<&str> = rotated.iter().map(AsRef::as_ref).collect();

            if let Some(col_idx) = get_reflection_point(&rotated_str) {
                return col_idx;
            }
            0
        })
        .sum();

    dbg!(r);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");
    let _test2 = std::fs::read_to_string("src/bin/test2").expect("file name input");

    process(_input);
}
