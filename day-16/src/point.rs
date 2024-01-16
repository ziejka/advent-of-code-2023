use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn get_diff(&self, p: &Point) -> (i8, i8) {
        (self.x as i8 - p.x as i8, self.y as i8 - p.y as i8)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.x, self.y)
    }
}
