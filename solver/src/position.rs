use std::convert::TryInto;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Position(i32);

impl From<usize> for Position {
    fn from(value: usize) -> Self {
        return Position(value.try_into().unwrap());
    }
}
