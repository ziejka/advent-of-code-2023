use std::{collections::HashMap, fmt::Display};

type PlatformType = Vec<Vec<char>>;
#[derive(Clone)]
struct Platform(PlatformType);

impl Platform {
    fn new(inner: &PlatformType) -> Self {
        Self(inner.clone())
    }

    fn move_up(&mut self) {
        let mut row_idx = 0;
        let mut col_idx = 0;
        while col_idx < self.0[0].len() {
            while row_idx < self.0.len() - 1 {
                row_idx += 1;
                let item = self.0[row_idx][col_idx];
                if item != 'O' {
                    continue;
                }

                self.0[row_idx][col_idx] = '.';
                let mut destination_idx = row_idx;
                while destination_idx > 0 {
                    destination_idx -= 1;
                    let element = self.0[destination_idx][col_idx];
                    if element == '#' || element == 'O' {
                        destination_idx += 1;
                        break;
                    }
                }

                self.0[destination_idx][col_idx] = 'O';
            }
            row_idx = 0;
            col_idx += 1;
        }
    }

    fn rotate(&mut self) {
        let old_platform: PlatformType = self.0.clone();
        let length = self.0[0].len();
        let mut row_idx = 0;
        let mut col_idx = 0;

        while col_idx < length {
            while row_idx < self.0.len() {
                self.0[col_idx][length - 1 - row_idx] = old_platform[row_idx][col_idx];

                row_idx += 1;
            }
            row_idx = 0;
            col_idx += 1;
        }
    }

    fn get_sum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let stones = row.iter().filter(|&&c| c == 'O').count();
                return stones * (self.0.len() - row_idx);
            })
            .sum()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            let s: String = row.iter().collect();
            writeln!(f, "{}", s)?;
        }
        return Ok(());
    }
}

fn process(s: String) {
    let p: PlatformType = s
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut cache: HashMap<String, Platform> = HashMap::new();
    let mut seen_order: HashMap<String, usize> = HashMap::new();
    let mut platform = Platform::new(&p);

    cache.insert(platform.to_string(), platform.clone());
    seen_order.insert(platform.to_string(), 0);

    let mut cycle_index = 0;
    let mut side_idx = 0;
    let mut order = 0;
    let mut is_done = false;

    while !is_done && cycle_index < 1000000000 {
        while side_idx < 4 {
            order += 1;
            platform.move_up();
            if cache.contains_key(&platform.to_string()) {
                let seen_idx = seen_order.get(&platform.to_string()).unwrap();
                let left_in_cycle = 4 - side_idx;
                let left_moves: i64 = (1000000000 - cycle_index) * 4;
                let diff = order - seen_idx;
                let m = left_moves % diff as i64;

                let end_pos = seen_idx + m as usize + left_in_cycle;
                let asd = seen_order
                    .iter()
                    .find(|(_, &val)| val == end_pos)
                    .unwrap()
                    .0;
                platform = cache.get(asd).unwrap().clone();

                println!(
                    "
                Found at {},
                side_idx: {}
                idx: {},
                left_moves: {},
                m {},
                asd {:?}
                order: {}",
                    seen_idx, side_idx, cycle_index, left_moves, m, asd, order
                );

                is_done = true;
                break;
            }

            cache.insert(platform.to_string(), platform.clone());
            seen_order.insert(platform.to_string(), order);

            platform.rotate();
            if cache.contains_key(&platform.to_string()) {
                println!(
                    "Found rotate at index {:?}",
                    seen_order.get(&platform.to_string())
                );
                is_done = true;
                break;
            }
            cache.insert(platform.to_string(), platform.clone());
            seen_order.insert(platform.to_string(), order);

            side_idx += 1;
        }
        side_idx = 0;
        cycle_index += 1;
    }

    dbg!(platform.get_sum());
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_test);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fn() {}
}
