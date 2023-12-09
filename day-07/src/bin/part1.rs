use std::collections::HashMap;

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

        if values.iter().any(|&&v| v == 5) {
            return Kind::FiveOfAKind;
        }
        if values.iter().any(|&&v| v == 4) {
            return Kind::FourOfAKind;
        }
        if values.iter().any(|&&v| v == 3) && values.iter().any(|&&v| v == 2) {
            return Kind::FullHouse;
        }
        if values.iter().any(|&&v| v == 3) {
            return Kind::TheeOfAKind;
        }

        let pairs = values.iter().filter(|&&&v| v == 2).count();
        if pairs == 2 {
            return Kind::TwoPairs;
        }
        if pairs == 1 {
            return Kind::Pair;
        }
        return Kind::HighCard;
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
            RoundWithKind { round: r, kind }
        })
        .collect::<Vec<_>>();

    let rank_mapping: HashMap<char, u32> = [
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]
    .iter()
    .cloned()
    .collect();

    rounds.sort_by(|a, b| {
        if a.kind == b.kind {
            for (a, b) in a.round.hand.chars().zip(b.round.hand.chars()) {
                if a != b {
                    let a_rank = rank_mapping.get(&a).unwrap();
                    let b_rank = rank_mapping.get(&b).unwrap();
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
