fn main() {
    let file = std::fs::read_to_string("src/bin/input").expect("file name input");
    process(file);
}

fn process(string: String) {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = ["0", "o1e", "t2e", "t3e", "4", "5", "6", "7n", "e8t", "n9e"];

    let result = string
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            for (digit, word) in digits.iter().zip(words.iter()) {
                line = line.replace(word, digit)
            }
            return line;
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|digits| 10 * digits.get(0).unwrap() + digits.last().unwrap())
        .sum::<u32>();

    println!("{:?}", result);
}
