use crate::{constraints::OverconstrainedError, position::Position};

#[derive(Clone, Copy)]
pub struct Range {
    pub min: Position,
    pub max: Position,
}

impl Range {
    pub fn choose_most_specific(left: Range, right: Range) -> Result<Range, OverconstrainedError> {
        let left_inside_right = left.min >= right.min && left.max <= right.max;
        let right_inside_left = left.min <= right.min && left.max >= right.max;

        if left_inside_right {
            Ok(left)
        } else if right_inside_left {
            Ok(right)
        } else {
            Err(OverconstrainedError)
        }
    }
}
