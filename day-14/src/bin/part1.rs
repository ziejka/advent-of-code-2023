fn _println_platform(platform: &[Vec<char>]) {
    platform.iter().for_each(|row| {
        let s: String = row.iter().collect();
        println!("{s}");
    });
    println!("");
}

fn process(s: String) {
    let mut platform: Vec<Vec<char>> = s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut row_idx = 0;
    let mut col_idx = 0;
    while col_idx < platform[0].len() {
        while row_idx < platform.len() - 1 {
            row_idx += 1;
            let item = platform[row_idx][col_idx];
            if item != 'O' {
                continue;
            }

            platform[row_idx][col_idx] = '.';
            let mut destination_idx = row_idx;
            while destination_idx > 0 {
                destination_idx -= 1;
                let element = platform[destination_idx][col_idx];
                if element == '#' || element == 'O' {
                    destination_idx += 1;
                    break;
                }
            }

            platform[destination_idx][col_idx] = 'O';
        }
        row_idx = 0;
        col_idx += 1;
    }

    let sum: usize = platform
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let stones_in_row = row.iter().filter(|&&c| c == 'O').count();
            return stones_in_row * (platform.len() - index);
        })
        .sum();

    dbg!(sum);
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
    fn test_fn() {}
}
