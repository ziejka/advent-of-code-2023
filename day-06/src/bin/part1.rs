fn calculate_race(time: u32, max_distance: u32) -> u32 {
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
    let race_data = s
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .flat_map(|n| n.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut winning: Vec<u32> = vec![];
    while i < race_data[0].len() {
        winning.push(calculate_race(race_data[0][i], race_data[1][i]));
        i += 1;
    }
    let result: u32 = winning.iter().fold(1, |acc, x| acc * x);
    println!("{:?}", result);
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
    fn test_calculate_race() {
        assert_eq!(calculate_race(7, 9), 4);
        assert_eq!(calculate_race(15, 40), 8);
        assert_eq!(calculate_race(30, 200), 9);
    }
}
