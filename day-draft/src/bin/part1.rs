fn parse_line(s: &str) -> (String, Vec<u32>) {
    let iterator = s.split(" ");
    let parts = iterator.next().unwrap();

    let lengths = iterator.

}

fn is_valid(s: &str, lengths: &Vec<u32>) -> bool {}

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
    fn test_fn() {
        assert_eq!("#.#.### 1,1,3", 1);
    }
}
