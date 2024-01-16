use std::{
    collections::{HashSet, VecDeque},
    fmt,
    str::FromStr,
};

use crate::point::Point;

pub struct Tiles {
    tiles: Vec<Vec<char>>,
    pub excited: HashSet<String>,
    visited: HashSet<String>,
}

impl FromStr for Tiles {
    type Err = fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            tiles: s.lines().map(|l| l.chars().collect()).collect(),
            excited: HashSet::new(),
            visited: HashSet::new(),
        })
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}
fn get_visited_key(from_point: &Point, to_point: &Point) -> String {
    format!("{}-{}", from_point.to_string(), to_point.to_string())
}

impl Tiles {
    pub fn calculate(&mut self) {
        let mut from_points = VecDeque::from([Point { x: 0, y: 0 }]);
        let mut points = VecDeque::new();
        if let Some(v) =
            self.get_new_point_from_char(&self.tiles[0][0], &Point { x: 0, y: 0 }, Direction::Left)
        {
            points.push_back(v[0].clone());
        }

        self.excited.insert(from_points[0].to_string());
        while let (Some(point), Some(from_point)) = (points.pop_front(), from_points.pop_front()) {
            if let Some(vec) = self.process_point(&from_point, &point) {
                for v in vec {
                    from_points.push_back(point.clone());
                    points.push_back(v);
                }
            }
        }
    }

    fn process_point(&mut self, from_point: &Point, to_point: &Point) -> Option<Vec<Point>> {
        self.excited.insert(to_point.to_string());
        let key = get_visited_key(from_point, to_point);
        if !self.visited.insert(key) {
            return None;
        }

        let p_diff = from_point.get_diff(&to_point);
        let from_dir = match p_diff {
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Up,
            (-0, 1) => Direction::Down,
            _ => Direction::None,
        };

        match self
            .tiles
            .get(to_point.y)
            .and_then(|row| row.get(to_point.x))
        {
            Some(c) => self.get_new_point_from_char(c, to_point, from_dir),
            None => None,
        }
    }

    fn get_new_point_from_char(
        &self,
        c: &char,
        point: &Point,
        from_dir: Direction,
    ) -> Option<Vec<Point>> {
        return match c {
            '-' => self.get_horizontal_points(point, from_dir),
            '|' => self.get_vertical_points(point, from_dir),
            '\\' => self.get_right_slide_points(point, from_dir),
            '/' => self.get_left_slide_points(point, from_dir),
            _ => self.get_default_points(point, from_dir),
        };
    }

    // Char: -
    fn get_horizontal_points(&self, point: &Point, from_dir: Direction) -> Option<Vec<Point>> {
        match from_dir {
            Direction::Left => self.get_points(point, vec![Self::get_right_point]),
            Direction::Right => self.get_points(point, vec![Self::get_left_point]),
            Direction::Up => {
                self.get_points(point, vec![Self::get_left_point, Self::get_right_point])
            }
            Direction::Down => {
                self.get_points(point, vec![Self::get_left_point, Self::get_right_point])
            }
            Direction::None => None,
        }
    }

    // Char |
    fn get_vertical_points(&self, point: &Point, from_dir: Direction) -> Option<Vec<Point>> {
        match from_dir {
            Direction::Left => {
                self.get_points(point, vec![Self::get_down_point, Self::get_up_point])
            }
            Direction::Right => {
                self.get_points(point, vec![Self::get_down_point, Self::get_up_point])
            }
            Direction::Up => self.get_points(point, vec![Self::get_down_point]),
            Direction::Down => self.get_points(point, vec![Self::get_up_point]),
            Direction::None => None,
        }
    }

    // Char \
    fn get_right_slide_points(&self, point: &Point, from_dir: Direction) -> Option<Vec<Point>> {
        match from_dir {
            Direction::Left => self.get_points(point, vec![Self::get_down_point]),
            Direction::Right => self.get_points(point, vec![Self::get_up_point]),
            Direction::Up => self.get_points(point, vec![Self::get_right_point]),
            Direction::Down => self.get_points(point, vec![Self::get_left_point]),
            Direction::None => None,
        }
    }

    // Char /
    fn get_left_slide_points(&self, point: &Point, from_dir: Direction) -> Option<Vec<Point>> {
        match from_dir {
            Direction::Left => self.get_points(point, vec![Self::get_up_point]),
            Direction::Right => self.get_points(point, vec![Self::get_down_point]),
            Direction::Up => self.get_points(point, vec![Self::get_left_point]),
            Direction::Down => self.get_points(point, vec![Self::get_right_point]),
            Direction::None => None,
        }
    }

    // Char .
    fn get_default_points(&self, point: &Point, from_dir: Direction) -> Option<Vec<Point>> {
        match from_dir {
            Direction::Left => self.get_points(point, vec![Self::get_right_point]),
            Direction::Right => self.get_points(point, vec![Self::get_left_point]),
            Direction::Up => self.get_points(point, vec![Self::get_down_point]),
            Direction::Down => self.get_points(point, vec![Self::get_up_point]),
            Direction::None => None,
        }
    }

    fn get_points(
        &self,
        point: &Point,
        funcs: Vec<fn(&Self, &Point) -> Option<Point>>,
    ) -> Option<Vec<Point>> {
        let mut v = vec![];
        for func in funcs {
            if let Some(p) = func(self, &point) {
                v.push(p)
            }
        }

        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }

    fn get_left_point(&self, point: &Point) -> Option<Point> {
        if point.x == 0 {
            return None;
        }
        Some({
            Point {
                x: point.x - 1,
                y: point.y,
            }
        })
    }

    fn get_right_point(&self, point: &Point) -> Option<Point> {
        if point.x == self.tiles[0].len() - 1 {
            return None;
        }
        Some(Point {
            x: point.x + 1,
            y: point.y,
        })
    }

    fn get_up_point(&self, point: &Point) -> Option<Point> {
        if point.y == 0 {
            return None;
        }
        Some(Point {
            x: point.x,
            y: point.y - 1,
        })
    }

    fn get_down_point(&self, point: &Point) -> Option<Point> {
        if point.y == self.tiles.len() - 1 {
            return None;
        }
        Some(Point {
            x: point.x,
            y: point.y + 1,
        })
    }
}
