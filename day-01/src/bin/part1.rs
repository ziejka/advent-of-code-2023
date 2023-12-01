fn main() {
    let file = std::fs::read_to_string("src/bin/input").expect("file name input");
    let result = file
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .map(|digits| 10 * digits.get(0).unwrap() + digits.last().unwrap())
        .sum::<u32>();

    println!("{:?}", result);
}
