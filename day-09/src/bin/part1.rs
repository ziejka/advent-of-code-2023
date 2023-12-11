use rayon::prelude::*;

fn process(s: String) {
    let data: Vec<Vec<i32>> = s
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .collect();

    let result: i32 = data
        .par_iter()
        .map(|vector| {
            let mut vectors = vec![vector.clone()];
            while vectors.last().unwrap().iter().any(|n| *n != 0) {
                let temp_v = vectors.last().unwrap();
                let subtracted = temp_v
                    .iter()
                    .skip(1)
                    .zip(temp_v.iter().take(temp_v.len() - 1))
                    .map(|(a, b)| a - b)
                    .collect::<Vec<_>>();

                vectors.push(subtracted);
            }
            let mut i = vectors.len() - 1;

            while i > 0 {
                let l1 = *vectors[i].last().unwrap();
                let l2 = *vectors[i - 1].last().unwrap();

                vectors[i - 1].push(l1 + l2);
                i -= 1;
            }

            let next_history_value = *vectors.first().unwrap().last().unwrap();
            next_history_value
        })
        .sum();

    println!("result: {}", result);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test2").expect("file name input");

    process(_input);
}
