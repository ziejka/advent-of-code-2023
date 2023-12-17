use std::collections::VecDeque;

fn _find_shortest_path(
    grid: &Vec<Vec<char>>,
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

fn calculate_distance_between_points(
    p1: (usize, usize),
    p2: (usize, usize),
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
) -> u64 {
    let universe_size = 999999;
    let mut dist = 0;
    if p1.0 > p2.0 {
        for i in p2.0..p1.0 {
            dist += 1;
            if empty_rows.contains(&i) {
                dist += universe_size;
            }
        }
    } else {
        for i in p1.0..p2.0 {
            dist += 1;
            if empty_rows.contains(&i) {
                dist += universe_size;
            }
        }
    }

    if p1.1 > p2.1 {
        for i in p2.1..p1.1 {
            dist += 1;
            if empty_columns.contains(&i) {
                dist += universe_size;
            }
        }
    } else {
        for i in p1.1..p2.1 {
            dist += 1;
            if empty_columns.contains(&i) {
                dist += universe_size;
            }
        }
    }

    return dist;
}

fn find_empty_rows(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_rows = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(i);
        }
    }

    empty_rows
}

fn find_empty_columns(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_columns = Vec::new();

    for i in 0..grid[0].len() {
        if grid.iter().all(|row| row[i] == '.') {
            empty_columns.push(i);
        }
    }

    empty_columns
}

fn process(s: String) {
    let star_map: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let empty_rows = find_empty_rows(&star_map);
    let empty_columns = find_empty_columns(&star_map);

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
            let dist =
                calculate_distance_between_points(stars[i], stars[j], &empty_rows, &empty_columns);
            distances.push(dist);
            j += 1;
        }
        i += 1;
    }

    let sum: u64 = distances.iter().sum();

    println!("s: {:?}", sum);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_input);
}
