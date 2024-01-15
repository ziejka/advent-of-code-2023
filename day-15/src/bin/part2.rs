use std::{collections::HashMap, fmt, str::FromStr};

fn calculate(s: &str) -> usize {
    let mut current = 0;

    for c in s.chars() {
        let ascii = c as usize;
        current += ascii;
        current *= 17;
        current = current % 256;
    }
    return current;
}

#[derive(Debug)]
struct Items {
    map: HashMap<String, u32>,
    order: Vec<String>,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            order: vec![],
        }
    }
}
#[derive(Debug)]
struct Facility {
    boxes: [Items; 256],
}

impl Facility {
    fn execute_command(&mut self, command: &Command) {
        command.execute(self);
    }

    fn get_total_focal_lense(&self) -> u32 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_idx, item)| {
                let mut result: u32 = 0;
                for (lense_idx, key) in item.order.iter().enumerate() {
                    if let Some(lense_size) = item.map.get(key) {
                        result += (1 + box_idx as u32) * (1 + lense_idx as u32) * lense_size;
                    }
                }
                result
            })
            .sum()
    }
}

fn process(s: String) {
    let items_vec: Vec<Items> = (0..256).map(|_| Items::default()).collect();
    let items: [Items; 256] = match items_vec.try_into() {
        Ok(array) => array,
        Err(vec) => panic!("Expected a Vec of length {} but it was {}", 256, vec.len()),
    };
    let mut boxes: Facility = Facility { boxes: items };

    s.split(",")
        .filter_map(|s| s.parse::<Command>().ok())
        .for_each(|command| boxes.execute_command(&command));

    println!("{:?}", boxes.get_total_focal_lense());
}

#[derive(Debug, PartialEq)]
enum Action {
    Remove,
    Add,
}

#[derive(Debug, PartialEq)]
struct Command {
    action: Action,
    box_idx: usize,
    lense_size: u32,
    label: String,
}

impl Command {
    fn execute(&self, boxes: &mut Facility) {
        let b = boxes
            .boxes
            .get_mut(self.box_idx)
            .expect("box item expected");
        match self.action {
            Action::Remove => {
                b.map.remove_entry(&self.label);
                if let Some(pos) = b.order.iter().position(|x| *x == self.label) {
                    b.order.remove(pos);
                }
            }
            Action::Add => {
                let new_entry = b.map.insert(self.label.clone(), self.lense_size);
                if new_entry.is_none() {
                    b.order.push(self.label.clone());
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut box_index_chars = vec![];
        let mut action = Action::Add;
        let mut focal_lense = 0;
        for c in s.chars() {
            match c {
                c if c.is_numeric() => {
                    focal_lense = c.to_string().parse().map_err(|_| fmt::Error)?
                }
                '=' => action = Action::Add,
                '-' => action = Action::Remove,
                _ => box_index_chars.push(c),
            }
        }
        let s: String = box_index_chars.iter().collect::<String>();
        let box_idx: usize = calculate(&s);

        Ok(Command {
            action,
            box_idx,
            lense_size: focal_lense,
            label: s,
        })
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
    fn test_create_step() {
        let step = "rn=1".parse::<Command>();
        assert_eq!(
            Ok(Command {
                action: Action::Add,
                box_idx: 0,
                lense_size: 1,
                label: "rn".to_string(),
            }),
            step
        );

        let step = "cm-".parse::<Command>();
        assert_eq!(
            Ok(Command {
                action: Action::Remove,
                box_idx: 0,
                lense_size: 0,
                label: "cm".to_string(),
            }),
            step
        );

        let step = "pc=4".parse::<Command>();
        assert_eq!(
            Ok(Command {
                action: Action::Add,
                box_idx: 3,
                lense_size: 4,
                label: "pc".to_string(),
            }),
            step
        );
    }
}
