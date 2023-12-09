fn calculate_race(time: u64, max_distance: u64) -> u64 {
    let mut hold_time = 1;
    let mut number_of_winning_races = 0;
    while hold_time < time {
        if max_distance < hold_time * (time - hold_time) {
            number_of_winning_races += 1;
        }
        hold_time += 1;
    }

    return number_of_winning_races;
}

fn process(s: String) {
    let s = s.replace(" ", "");

    let race_data = s
        .lines()
        .map(|line| {
            line.split(":")
                .skip(1)
                .flat_map(|n| n.parse::<u64>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut winning: Vec<u64> = vec![];

    while i < race_data[0].len() {
        winning.push(calculate_race(race_data[0][i], race_data[1][i]));
        i += 1;
    }
    let result: u64 = winning.iter().fold(1, |acc, x| acc * x);
    println!("{:?}", result);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);
}
