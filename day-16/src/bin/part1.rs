use day_16::{
    point::Point,
    tiles::{Direction, Tiles},
};

fn process(s: String) {
    if let Ok(mut tiles) = s.parse::<Tiles>() {
        let p = Point { x: 0, y: 0 };
        tiles.calculate(&p, Direction::Left);
        let mut excited = tiles.excited.iter().collect::<Vec<&String>>();
        excited.sort_by(|a, b| a.cmp(b));
        println!("{:?}", excited.len());
    }
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
