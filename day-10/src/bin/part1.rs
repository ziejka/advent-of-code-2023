use std::{collections::HashMap, usize};

#[derive(Debug, PartialEq, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn get_coordinates(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Move::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Move::Down => Some((x, y + 1)),
            Move::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Move::Right => Some((x + 1, y)),
        }
    }

    fn get_opposite(&self) -> Self {
        match self {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}

trait FromChar {
    type Err;
    fn from_char(c: &char) -> Result<Self, Self::Err>
    where
        Self: Sized;
}

impl FromChar for Vec<Move> {
    type Err = anyhow::Error;

    fn from_char(s: &char) -> Result<Self, Self::Err> {
        match s {
            '|' => Ok(vec![Move::Up, Move::Down]),
            '-' => Ok(vec![Move::Left, Move::Right]),
            'L' => Ok(vec![Move::Right, Move::Up]),
            'J' => Ok(vec![Move::Left, Move::Up]),
            '7' => Ok(vec![Move::Left, Move::Down]),
            'F' => Ok(vec![Move::Right, Move::Down]),
            '.' => Ok(vec![]),
            'S' => Ok(vec![Move::Up, Move::Down, Move::Left, Move::Right]),
            _ => Err(anyhow::anyhow!("Invalid char")),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    moves: Option<Vec<Move>>,
    position: (usize, usize),
}

impl Node {
    fn new(s: &char, position: (usize, usize)) -> Self {
        let moves = Vec::<Move>::from_char(s).ok();
        Node { moves, position }
    }

    fn can_enter_from(&self, m: &Move) -> bool {
        if let Some(ref moves) = self.moves {
            moves.contains(&m.get_opposite())
        } else {
            false
        }
    }

    fn remove_move(&mut self, m: &Move) {
        if let Some(ref mut moves) = self.moves {
            if let Some(pos) = moves.iter().position(|x| *x == *m) {
                moves.remove(pos);
            }
        }
    }
}

fn process(s: String) {
    let mut position = (0, 0);
    let mut items = 0;

    let maze = s
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        position = (x, y);
                    }
                    items += 1;
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let c = maze.get(position.1).unwrap().get(position.0).unwrap();
    let current_node = Node::new(c, position);

    let mut current_nodes = vec![current_node.clone()];
    let mut visited: HashMap<String, Node> = HashMap::new();
    let key = format!("{}{}", position.0, position.1);

    visited.insert(key, current_node);
    let mut count = 0;

    loop {
        count += 1;
        if count > items {
            println!("Didn't find exit");
            return;
        }
        let mut temp_nodes = vec![];

        for node in &current_nodes {
            if let Some(ref moves) = node.moves {
                for m in moves.iter() {
                    if let Some((x, y)) = m.get_coordinates(node.position) {
                        let c = maze.get(y).unwrap().get(x).unwrap();
                        let key = format!("{}-{}", x, y);

                        if let Some(node) = visited.get_mut(&key) {
                            if node.can_enter_from(&m) {
                                println!("Found exit at {count}");
                                return;
                            }
                        } else {
                            let mut node = Node::new(c, (x, y));
                            if node.can_enter_from(m) {
                                node.remove_move(&m.get_opposite());
                                temp_nodes.push(node.clone());
                                visited.insert(key, node);
                            }
                        }
                    }
                }
            }
        }
        current_nodes = temp_nodes;
    }
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);
}
