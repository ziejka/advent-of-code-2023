use std::time::Instant;

use day_16::{
    point::Point,
    tiles::{Direction, Tiles},
};
use rayon::prelude::*;

fn process(s: String) {
    if let Ok(tiles) = s.parse::<Tiles>() {
        let result = [
            Direction::Left,
            Direction::Right,
            Direction::Top,
            Direction::Down,
        ]
        .par_iter()
        .filter_map(|direction| match direction {
            Direction::Left => (0..tiles.tiles.len())
                .into_par_iter()
                .map(|y| {
                    let mut tmp = tiles.clone();
                    let p = Point { x: 0, y };
                    tmp.calculate(&p, Direction::Left);
                    tmp.excited.len()
                })
                .max(),
            Direction::Right => (0..tiles.tiles.len())
                .into_par_iter()
                .map(|y| {
                    let mut tmp = tiles.clone();
                    let p = Point {
                        x: tiles.tiles[0].len() - 1,
                        y,
                    };
                    tmp.calculate(&p, Direction::Right);
                    tmp.excited.len()
                })
                .max(),
            Direction::Top => (0..tiles.tiles[0].len())
                .into_par_iter()
                .map(|x| {
                    let mut tmp = tiles.clone();
                    let p = Point { x, y: 0 };
                    tmp.calculate(&p, Direction::Top);
                    tmp.excited.len()
                })
                .max(),
            Direction::Down => (0..tiles.tiles[0].len())
                .into_par_iter()
                .map(|x| {
                    let mut tmp = tiles.clone();
                    let p = Point {
                        x,
                        y: tiles.tiles.len() - 1,
                    };
                    tmp.calculate(&p, Direction::Down);
                    tmp.excited.len()
                })
                .max(),
            Direction::None => None,
        })
        .max();
        println!("{:?}", result);
    }
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    let stat = Instant::now();
    process(_input);
    let duration = stat.elapsed();
    println!("Elapse {:?}", duration);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fn() {}
}
