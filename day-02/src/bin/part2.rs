use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq, Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

enum ColorValue {
    Color(Color, u32),
}

impl ColorValue {
    fn get_value_tuple(&self) -> (Color, u32) {
        match self {
            ColorValue::Color(c, v) => (c.clone(), *v),
        }
    }

    fn tuple_to_color(tuple: (u32, &str)) -> Option<Self> {
        match tuple {
            (n, "red") => Some(ColorValue::Color(Color::Red, n)),
            (n, "green") => Some(ColorValue::Color(Color::Green, n)),
            (n, "blue") => Some(ColorValue::Color(Color::Blue, n)),
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

fn get_game_tuple(s: &str) -> Option<Vec<Vec<(Color, u32)>>> {
    let mut iterator = s.split(':');
    let rest = iterator
        .nth(1)?
        .split(';')
        .map(|game_round| {
            game_round
                .split(",")
                .filter_map(get_color_tuple)
                .filter_map(ColorValue::tuple_to_color)
                .map(|color| color.get_value_tuple())
                .collect::<Vec<(Color, u32)>>()
        })
        .collect::<Vec<Vec<(Color, u32)>>>();
    return Some(rest);
}

fn find_color(round: &[(Color, u32)], default_color: Color) -> (Color, u32) {
    match round.iter().find(|&&(color, _)| color == default_color) {
        Some(&(color, value)) => (color, value),
        None => (default_color, 1),
    }
}

fn update_color_value(acc: &mut HashMap<Color, u32>, color_value: (Color, u32)) {
    acc.entry(color_value.0)
        .and_modify(|value| {
            if color_value.1 > *value {
                *value = color_value.1;
            }
        })
        .or_insert(color_value.1);
}

fn process(string: String) -> u32 {
    return string
        .lines()
        .filter_map(get_game_tuple)
        .map(|rounds| {
            let result = HashMap::<Color, u32>::new();

            let colors = [Color::Red, Color::Green, Color::Blue];

            let x = rounds.iter().fold(result, |mut acc, round| {
                for color in &colors {
                    let color_value = find_color(&round, *color);
                    update_color_value(&mut acc, color_value);
                }
                acc
            });

            let round_result = x.iter().map(|(_, v)| v).fold(1, |acc, v| acc * v);

            return round_result;
        })
        .sum();
}

fn main() {
    let file = std::fs::read_to_string("src/bin/input").expect("file name input");

    let result = process(file);
    println!("{:?}", result);
}
