use std::collections::HashMap;

type NumberIndexes = Vec<Vec<bool>>;
type SymbolIndexes = Vec<Vec<isize>>;

fn insert_number_index(vec: &mut NumberIndexes, line_idx: usize, char_idx: usize) {
    if vec.len() <= line_idx {
        vec.resize(line_idx + 1, vec![]);
    }
    if vec[line_idx].len() <= char_idx {
        vec[line_idx].resize(char_idx + 1, false);
    }
    vec[line_idx][char_idx] = true;
}

fn insert_symbol_index(vec: &mut SymbolIndexes, line_idx: usize, char_idx: usize) {
    if vec.len() <= line_idx {
        vec.resize(line_idx + 1, vec![]);
    }
    vec[line_idx].push(char_idx as isize);
}

fn get_key(number_string: &String, line_idx: usize, char_idx: usize) -> String {
    format!("{number_string}:{line_idx}x{char_idx}")
}

fn get_number_from_key(key: &String) -> Option<usize> {
    key.split(':').next()?.parse::<usize>().ok()
}

fn process(str: String) -> usize {
    let mut number_indexes_map = HashMap::<String, NumberIndexes>::new();
    let mut symbol_indexes: SymbolIndexes = vec![];

    for (line_idx, line) in str.lines().enumerate() {
        let mut char_indexes: NumberIndexes = vec![];
        let mut number_str = String::new();

        for (char_idx, char) in line.chars().enumerate() {
            if char == '*' {
                insert_symbol_index(&mut symbol_indexes, line_idx, char_idx);
            }

            if char.is_ascii_alphanumeric() {
                number_str.push_str(&char.to_string());

                // check characters before char index
                if char_indexes.is_empty() {
                    if line_idx > 0 && char_idx > 0 {
                        insert_number_index(&mut char_indexes, line_idx - 1, char_idx - 1);
                    }
                    if char_idx > 0 {
                        insert_number_index(&mut char_indexes, line_idx, char_idx - 1);
                        insert_number_index(&mut char_indexes, line_idx + 1, char_idx - 1);
                    }
                }

                if line_idx > 0 {
                    insert_number_index(&mut char_indexes, line_idx - 1, char_idx);
                }
                insert_number_index(&mut char_indexes, line_idx, char_idx);
                insert_number_index(&mut char_indexes, line_idx + 1, char_idx);

                if char_idx == line.len() - 1 {
                    if line_idx > 0 {
                        insert_number_index(&mut char_indexes, line_idx - 1, char_idx);
                    }
                    insert_number_index(&mut char_indexes, line_idx, char_idx);
                    insert_number_index(&mut char_indexes, line_idx + 1, char_idx);

                    //check characters after char index
                    insert_number_index(&mut char_indexes, line_idx, char_idx + 1);
                    insert_number_index(&mut char_indexes, line_idx + 1, char_idx + 1);

                    number_indexes_map.insert(
                        get_key(&number_str, line_idx, char_idx),
                        char_indexes.clone(),
                    );
                    number_str.clear();
                    char_indexes.clear();
                }
            } else if !number_str.is_empty() {
                if line_idx > 0 {
                    insert_number_index(&mut char_indexes, line_idx - 1, char_idx);
                }
                insert_number_index(&mut char_indexes, line_idx, char_idx);
                insert_number_index(&mut char_indexes, line_idx + 1, char_idx);

                number_indexes_map.insert(
                    get_key(&number_str, line_idx, char_idx - 1),
                    char_indexes.clone(),
                );
                number_str.clear();
                char_indexes.clear();
            }
        }
    }

    let mut result = 0;
    for (y, row_indexes) in symbol_indexes.iter().enumerate() {
        for x in row_indexes {
            let a = number_indexes_map
                .iter()
                .filter_map(|(key, number_indexes)| {
                    if let Some(row) = number_indexes.get(y as usize) {
                        if let Some(&value) = row.get(*x as usize) {
                            if value {
                                return get_number_from_key(key);
                            }
                        }
                    }
                    return None;
                })
                .collect::<Vec<usize>>();

            if a.len() == 2 {
                result += a[0] * a[1];
            }
        }
    }
    return result;
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let result = process(_input);
    println!("{:?}", result);
}
