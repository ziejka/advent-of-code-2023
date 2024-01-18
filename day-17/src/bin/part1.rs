fn process(s: String) {}

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
