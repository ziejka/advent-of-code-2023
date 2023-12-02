#[derive(PartialEq, Debug)]
enum Colors {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Colors {
    fn is_less_then_max(&self) -> bool {
        match self {
            Colors::Red(value) => value < &13,
            Colors::Green(value) => value < &14,
            Colors::Blue(value) => value < &15,
        }
    }

    fn tuple_to_color(tuple: (u32, &str)) -> Option<Self> {
        match tuple {
            (n, "red") => Some(Colors::Red(n)),
            (n, "green") => Some(Colors::Green(n)),
            (n, "blue") => Some(Colors::Blue(n)),
            _ => None,
        }
    }
}

fn get_color_tuple(s: &str) -> Option<(u32, &str)> {
    let mut iterator = s.split_whitespace();
    let n = iterator.next()?.parse::<u32>().ok()?;
    let s = iterator.next()?;

    return Some((n, &s));
}

fn get_game_tuple(s: &str) -> Option<(u32, bool)> {
    let mut iterator = s.split(':');
    let idx = iterator
        .next()?
        .split_whitespace()
        .last()?
        .parse::<u32>()
        .ok()?;

    let rest = iterator
        .next()?
        .split(';')
        .map(|game_round| {
            game_round
                .split(",")
                .filter_map(get_color_tuple)
                .filter_map(Colors::tuple_to_color)
                .map(|color| color.is_less_then_max())
                .collect::<Vec<bool>>()
        })
        .flatten()
        .all(std::convert::identity);
    return Some((idx, rest));
}

fn process(string: String) -> u32 {
    return string
        .lines()
        .filter_map(get_game_tuple)
        .filter_map(|(idx, is_valid)| {
            if is_valid {
                return Some(idx);
            } else {
                None
            }
        })
        .sum();
}

fn main() {
    let file = std::fs::read_to_string("src/bin/input").expect("file name input");

    let result = process(file);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuple_to_color_ok() {
        let tuple = (2, "red");
        assert_eq!(Colors::tuple_to_color(tuple), Some(Colors::Red(2)))
    }

    #[test]
    fn tuple_to_color_invalid() {
        let tuple = (2, "reddish");
        assert_eq!(Colors::tuple_to_color(tuple), None)
    }

    #[test]
    fn test_get_color_tuple_valid() {
        let s = "2 red";
        assert_eq!(get_color_tuple(s), Some((2, "red")))
    }

    #[test]
    fn test_get_color_tuple_invalid_integer() {
        let s = "two red";
        assert_eq!(get_color_tuple(s), None)
    }

    #[test]
    fn test_get_color_tuple_invalid_string() {
        let s = "red";
        assert_eq!(get_color_tuple(s), None)
    }

    #[test]
    fn test_is_less_max_red() {
        let color = Colors::Red(12);
        assert_eq!(color.is_less_then_max(), true);
    }

    #[test]
    fn test_is_less_max_green() {
        let color = Colors::Green(13);
        assert_eq!(color.is_less_then_max(), true);
    }

    #[test]
    fn test_is_less_max_blue() {
        let color = Colors::Blue(14);
        assert_eq!(color.is_less_then_max(), true);
    }

    #[test]
    fn test_is_less_max_red_false() {
        let color = Colors::Red(13);
        assert_eq!(color.is_less_then_max(), false);
    }

    #[test]
    fn test_is_less_max_green_false() {
        let color = Colors::Green(14);
        assert_eq!(color.is_less_then_max(), false);
    }

    #[test]
    fn test_is_less_max_blue_false() {
        let color = Colors::Blue(15);
        assert_eq!(color.is_less_then_max(), false);
    }
}
