use rayon::prelude::*;

fn calculate(s: &str) -> u32 {
    let mut current = 0;

    for c in s.chars() {
        let ascii = c as u32;
        current += ascii;
        current *= 17;
        current = current % 256;
    }
    return current;
}

fn process(s: String) {
    let result: u32 = s
        .split(",")
        .collect::<Vec<&str>>()
        .into_par_iter()
        .map(|s| calculate(s))
        .sum();

    println!("{result}");
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input.to_string());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fn() {}
}
