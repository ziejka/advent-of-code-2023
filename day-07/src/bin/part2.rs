use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref RANK_MAPPING: HashMap<char, u32> = {
        let mut m = HashMap::new();
        m.insert('A', 13);
        m.insert('K', 12);
        m.insert('Q', 11);
        m.insert('T', 9);
        m.insert('9', 8);
        m.insert('8', 7);
        m.insert('7', 6);
        m.insert('6', 5);
        m.insert('5', 4);
        m.insert('4', 3);
        m.insert('3', 2);
        m.insert('2', 1);
        m.insert('J', 0);
        m
    };
}

#[derive(Debug)]
struct Round {
    hand: String, // set size to 5
    bid: u32,
}

#[derive(Debug)]
struct RoundWithKind {
    round: Round,
    kind: Kind,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Kind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    TheeOfAKind,
    TwoPairs,
    Pair,
    HighCard,
}

trait HasTwoPairs {
    fn has_two_pairs(&self) -> bool;
}

trait HasValue {
    fn has_value(&self, value: u32) -> bool;
}

impl HasTwoPairs for Vec<&u32> {
    fn has_two_pairs(&self) -> bool {
        self.iter().filter(|&&&v| v == 2).count() == 2
    }
}

impl HasValue for Vec<&u32> {
    fn has_value(&self, value: u32) -> bool {
        self.iter().any(|&&v| v == value)
    }
}

impl Round {
    pub fn get_kind(&self) -> Kind {
        let mut kind: HashMap<char, u32> = HashMap::new();

        self.hand.chars().for_each(|c| {
            if let Some(value) = kind.get_mut(&c) {
                *value += 1;
            } else {
                kind.insert(c, 1);
            }
        });

        let values: Vec<_> = kind.values().collect();
        let max = values.iter().max().unwrap();

        if let Some(jokers) = kind.get(&'J') {
            match jokers {
                5 => return Kind::FiveOfAKind,
                4 => return Kind::FiveOfAKind,
                3 => {
                    if values.has_value(2) {
                        return Kind::FiveOfAKind;
                    }
                    return Kind::FourOfAKind;
                }
                2 => {
                    if values.has_value(3) {
                        return Kind::FiveOfAKind;
                    }
                    if values.has_two_pairs() {
                        return Kind::FourOfAKind;
                    }
                    return Kind::TheeOfAKind;
                }
                1 => match max {
                    4 => return Kind::FiveOfAKind,
                    3 => return Kind::FourOfAKind,
                    2 => {
                        if values.has_two_pairs() {
                            return Kind::FullHouse;
                        }
                        return Kind::TheeOfAKind;
                    }
                    _ => return Kind::Pair,
                },
                _ => return Kind::Pair,
            }
        }

        match max {
            5 => return Kind::FiveOfAKind,
            4 => return Kind::FourOfAKind,
            3 => {
                if values.has_value(2) {
                    return Kind::FullHouse;
                }
                return Kind::TheeOfAKind;
            }
            2 => {
                if values.has_two_pairs() {
                    return Kind::TwoPairs;
                }
                return Kind::Pair;
            }
            _ => {
                return Kind::HighCard;
            }
        }
    }
}

fn process(s: String) {
    let mut rounds = s
        .lines()
        .map(|line| {
            let line_data: Vec<&str> = line.split_whitespace().collect();
            Round {
                hand: line_data[0].to_string(),
                bid: line_data[1].parse::<u32>().expect("number"),
            }
        })
        .map(|r| {
            let kind = r.get_kind();
            if r.hand.contains('J') {
                println!("{:?} {:?}", kind, r.hand);
            }
            RoundWithKind { round: r, kind }
        })
        .collect::<Vec<_>>();

    rounds.sort_by(|a, b| {
        if a.kind == b.kind {
            for (a, b) in a.round.hand.chars().zip(b.round.hand.chars()) {
                if a != b {
                    let a_rank = RANK_MAPPING.get(&a).unwrap();
                    let b_rank = RANK_MAPPING.get(&b).unwrap();
                    // revert order to get descending order
                    return a_rank.cmp(&b_rank);
                }
            }
        }
        return b.kind.cmp(&a.kind);
    });

    let result = rounds
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, cur)| acc + cur.round.bid * (idx as u32 + 1));

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
    fn test_fn() {
        let mut r = Round {
            hand: "JJJJJ".to_string(),
            bid: 1,
        };

        assert_eq!(r.get_kind(), Kind::FiveOfAKind);

        r.hand = "JJJJT".to_string();
        assert_eq!(r.get_kind(), Kind::FiveOfAKind);

        r.hand = "JJJTT".to_string();
        assert_eq!(r.get_kind(), Kind::FiveOfAKind);

        r.hand = "JJTTT".to_string();
        assert_eq!(r.get_kind(), Kind::FiveOfAKind);

        r.hand = "JTTTT".to_string();
        assert_eq!(r.get_kind(), Kind::FiveOfAKind);

        r.hand = "TTTJA".to_string();
        assert_eq!(r.get_kind(), Kind::FourOfAKind);

        r.hand = "TTJJA".to_string();
        assert_eq!(r.get_kind(), Kind::FourOfAKind);

        r.hand = "TJJJA".to_string();
        assert_eq!(r.get_kind(), Kind::FourOfAKind);

        r.hand = "TTJAA".to_string();
        assert_eq!(r.get_kind(), Kind::FullHouse);

        r.hand = "TTJSA".to_string();
        assert_eq!(r.get_kind(), Kind::TheeOfAKind);

        r.hand = "T2JSA".to_string();
        assert_eq!(r.get_kind(), Kind::Pair);
    }
}
