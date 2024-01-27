use std::collections::VecDeque;

fn _find_shortest_path(
    grid: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<u32> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<((usize, usize), u32)> = VecDeque::new();
    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    visited[start.0][start.1] = true;
    queue.push_back((start, 0));

    while let Some(((x, y), dist)) = queue.pop_front() {
        if (x, y) == end {
            return Some(dist - 1);
        }

        for (dx, dy) in &directions {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx < 0 || ny < 0 {
                continue;
            }

            if nx >= grid.len() as i32 || ny >= grid[0].len() as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if visited[nx][ny] {
                continue;
            }

            visited[nx][ny] = true;
            queue.push_back(((nx, ny), dist + 1));
        }
    }
    None
}

fn calculate_distance_between_points(p1: (usize, usize), p2: (usize, usize)) -> u32 {
    let x = (p1.0 as i32 - p2.0 as i32).abs();
    let y = (p1.1 as i32 - p2.1 as i32).abs();

    (x + y) as u32
}

fn find_empty_rows(grid: &[Vec<char>]) -> Vec<usize> {
    let mut empty_rows = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(i);
        }
    }

    empty_rows
}

fn find_empty_columns(grid: &[Vec<char>]) -> Vec<usize> {
    let mut empty_columns = Vec::new();

    for i in 0..grid[0].len() {
        if grid.iter().all(|row| row[i] == '.') {
            empty_columns.push(i);
        }
    }

    empty_columns
}

fn insert_empty_columns(grid: &mut Vec<Vec<char>>, empty_columns: &[usize]) {
    for row in grid.iter_mut() {
        for (num_idx, i) in empty_columns.iter().enumerate() {
            row.insert(*i + num_idx, '.');
        }
    }
}

fn insert_empty_rows(grid: &mut Vec<Vec<char>>, empty_rows: &[usize]) {
    for (idx, i) in empty_rows.iter().enumerate() {
        let row = vec!['.'; grid[0].len()];
        grid.insert(*i + idx, row);
    }
}

fn process(s: String) {
    let mut star_map: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let empty_rows = find_empty_rows(&star_map);
    insert_empty_rows(&mut star_map, &empty_rows);

    let empty_columns = find_empty_columns(&star_map);
    insert_empty_columns(&mut star_map, &empty_columns);

    let mut stars = vec![];
    star_map.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, c)| {
            if c != &'.' {
                stars.push((x, y));
            }
        })
    });

    let mut distances = vec![];
    let mut i = 0;
    while i < stars.len() - 1 {
        let mut j = i + 1;
        while j < stars.len() {
            let dist = calculate_distance_between_points(stars[i], stars[j]);
            distances.push(dist);
            j += 1;
        }
        i += 1;
    }

    let sum: u32 = distances.iter().sum();

    println!("s: {:?}", sum);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);
}
