use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    vec,
};

#[derive(PartialEq, Eq, Debug)]
struct State {
    cost: u32,
    point: Point,
    history: History,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}-{}", self.point.0, self.point.1, self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait GetFromPosition<T> {
    fn get_at_position(&self, point: &Point) -> T;
    fn set_at_position(&mut self, point: &Point, val: T);
}

impl GetFromPosition<u32> for Vec<Vec<u32>> {
    fn get_at_position(&self, point: &Point) -> u32 {
        self[point.1][point.0]
    }

    fn set_at_position(&mut self, point: &Point, val: u32) {
        self[point.1][point.0] = val
    }
}

trait GetNeighbors<T> {
    fn get_neighbors(&self, point: &Point, history: &History) -> Vec<(Point, T)>;
}

impl GetNeighbors<u32> for Vec<Vec<u32>> {
    fn get_neighbors(&self, point: &Point, history: &History) -> Vec<(Point, u32)> {
        let mut v = vec![];
        let possible_side = match history.get_blocked_side(4) {
            // 1,1 2,1 3,1 -> 3, (0/2)
            Some(1) => vec![(0, 1), (0, -1)],
            // 1,1 1,2 1,3 -> (0/2),3
            Some(0) => vec![(1, 0), (-1, 0)],
            None | Some(_) => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        };

        for (d_x, d_y) in possible_side {
            let new_x: i32 = point.0 as i32 - d_x;
            let new_y: i32 = point.1 as i32 - d_y;

            if new_x < 0 || new_x >= self[0].len() as i32 {
                continue;
            }
            if new_y < 0 || new_y >= self.len() as i32 {
                continue;
            }

            let new_point = Point(new_x as usize, new_y as usize);

            if history.items.len() > 2
                && Some(&new_point) == history.items.get(history.items.len() - 2)
            {
                continue;
            }
            let val = self.get_at_position(&new_point);

            v.push((new_point, val));
        }

        return v;
    }
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Debug)]
struct Point(usize, usize);

#[derive(Clone, PartialEq, Eq, Debug)]
struct History {
    items: Vec<Point>,
}

impl History {
    fn new() -> Self {
        Self { items: vec![] }
    }

    fn get_last_n_items(&self, n: usize) -> &[Point] {
        let start = if self.items.len() > n {
            self.items.len() - n
        } else {
            0
        };
        &self.items[start..]
    }

    fn get_blocked_side(&self, n: usize) -> Option<usize> {
        let h_vec = self.get_last_n_items(n);
        if h_vec.len() != n {
            return None;
        }

        let &Point(first_x, first_y) = h_vec.first()?;

        let (all_x_same, all_y_same) = h_vec
            .iter()
            .fold((true, true), |(x_same, y_same), &Point(x, y)| {
                (x_same && x == first_x, y_same && y == first_y)
            });

        if all_x_same {
            Some(0)
        } else if all_y_same {
            Some(1)
        } else {
            None
        }
    }
}

fn shortest_path(points: &mut Vec<Vec<u32>>) -> Option<u32> {
    let start = Point(0, 0);
    let goal = Point(points[0].len() - 1, points.len() - 1);
    // let mut dist: Vec<Vec<u32>> = points
    //     .iter()
    //     .map(|row| row.iter().map(|_| u32::MAX).collect())
    //     .collect();

    let mut visited: HashSet<String> = HashSet::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    // dist.set_at_position(&start, 0);
    let start_state = State {
        cost: 0,
        point: start.clone(),
        history: History::new(),
    };
    heap.push(start_state);

    while let Some(mut current_state) = heap.pop() {
        current_state
            .history
            .items
            .push(current_state.point.clone());

        if current_state.point == goal {
            // println!("Current {:?}", current_state.point);
            // let mut print_map: Vec<Vec<_>> = points
            //     .iter()
            //     .map(|r| r.iter().map(|_| '.').collect())
            //     .collect();

            // current_state
            //     .history
            //     .items
            //     .iter()
            //     .enumerate()
            //     .for_each(|(_, p)| print_map[p.1][p.0] = '#');

            // print_map.iter().for_each(|r| {
            //     r.iter().for_each(|x| {
            //         print!("{}", x);
            //     });
            //     print!("\n");
            // });

            return Some(current_state.cost);
        }

        if !visited.insert(current_state.to_string().clone()) {
            // println!("{current_state}");
            continue;
        }
        // if current_state.cost > dist.get_at_position(&current_state.point) {
        //     continue;
        // }

        for (new_point, c) in points.get_neighbors(&current_state.point, &current_state.history) {
            let new_history = History {
                items: current_state.history.get_last_n_items(4).into(),
            };

            let next = State {
                cost: current_state.cost + c,
                point: new_point.clone(),
                history: new_history,
            };

            // if next.cost < dist.get_at_position(&next.point) {
            //     dist.set_at_position(&next.point, next.cost);
            //     heap.push(next);
            // }
            heap.push(next);
        }
    }
    None
}

fn process(s: String) {
    let mut points: Vec<Vec<u32>> = s
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_string().parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect();
    let res = shortest_path(&mut points);
    println!("{:?}", res);
}

fn main() {
    let _input = std::fs::read_to_string("src/bin/input").expect("file name input");
    let _test = std::fs::read_to_string("src/bin/test").expect("file name input");

    process(_test);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_point_equal() {
        assert!(Point(1, 2) == Point(1, 2))
    }

    #[test]
    fn test_get_last_history() {
        let mut his = History {
            items: vec![Point(0, 0), Point(1, 0), Point(1, 1), Point(1, 2)],
        };
        assert_eq!(
            vec![Point(1, 0), Point(1, 1), Point(1, 2)],
            his.get_last_n_items(3)
        );

        his.items.remove(0);
        his.items.remove(0);
        assert_eq!(vec![Point(1, 1), Point(1, 2)], his.get_last_n_items(3));

        his.items.remove(0);
        his.items.remove(0);
        let expect: Vec<Point> = vec![];
        assert_eq!(expect, his.get_last_n_items(3));
    }

    #[test]
    fn test_his_diff() {
        let mut his = History {
            items: vec![Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)],
        };

        assert_eq!(Some(0), his.get_blocked_side(3));

        his.items = vec![Point(0, 0), Point(1, 1), Point(2, 1), Point(3, 1)];
        assert_eq!(Some(1), his.get_blocked_side(3));

        his.items = vec![Point(0, 0), Point(1, 2), Point(2, 1), Point(3, 1)];
        assert_eq!(None, his.get_blocked_side(3));

        his.items = vec![Point(2, 1), Point(3, 1)];
        assert_eq!(None, his.get_blocked_side(3));
    }
}
