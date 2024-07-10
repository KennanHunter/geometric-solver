use std::fmt::Debug;

use crate::{parametric_value::ParametricValue, position::Position};

#[derive(Debug)]
pub struct Point(u16);

impl Point {
    pub fn origin() -> Self {
        todo!();
    }
}

#[derive(Debug)]
pub struct PointDescriptor {
    pub x: ParametricValue,
    pub y: ParametricValue,
}

impl PointDescriptor {
    pub fn new(x: ParametricValue, y: ParametricValue) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parametric_value::ParametricValue, position::Position};

    #[test]
    fn serialize_literal() {
        let lit = ParametricValue::Literal(Position::from(0));

        assert_eq!(format!("{:#?}", lit), "")
    }
}
