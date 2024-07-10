use crate::point::Point;

pub struct World {
    pub points: Vec<Point>,
}

impl World {
    pub fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    pub fn render_points() {}
}
