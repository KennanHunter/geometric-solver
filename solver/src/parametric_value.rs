use std::fmt::Debug;

use crate::{constraints::OverconstrainedError, position::Position, range::Range};

pub enum ParametricValue {
    Literal(Position),
    Range(Range),
    Implicit,
}

impl ParametricValue {
    /// Return a value that describes the
    ///
    pub fn choose_most_specific(
        left: ParametricValue,
        right: ParametricValue,
    ) -> Result<ParametricValue, OverconstrainedError> {
        Ok(match (left, right) {
            (ParametricValue::Literal(left_position), ParametricValue::Literal(right_position)) => {
                if left_position == right_position {
                    ParametricValue::Literal(left_position)
                } else {
                    return Err(OverconstrainedError);
                }
            }

            (ParametricValue::Literal(literal), ParametricValue::Range(Range { min, max })) => {
                let literal_inside_range = literal >= min && literal <= max;

                if literal_inside_range {
                    ParametricValue::Literal(literal)
                } else {
                    return Err(OverconstrainedError);
                }
            }

            (ParametricValue::Literal(literal), ParametricValue::Implicit) => {
                ParametricValue::Literal(literal)
            }

            (ParametricValue::Range(Range { min, max }), ParametricValue::Literal(literal)) => {
                let literal_inside_range = literal >= min && literal <= max;

                if literal_inside_range {
                    ParametricValue::Literal(literal)
                } else {
                    return Err(OverconstrainedError);
                }
            }

            (ParametricValue::Range(left), ParametricValue::Range(right)) => {
                ParametricValue::Range(Range::choose_most_specific(left, right)?)
            }

            (ParametricValue::Range(range), ParametricValue::Implicit) => {
                ParametricValue::Range(range)
            }

            (ParametricValue::Implicit, ParametricValue::Literal(literal)) => {
                ParametricValue::Literal(literal)
            }

            (ParametricValue::Implicit, ParametricValue::Range(range)) => {
                ParametricValue::Range(range)
            }

            (ParametricValue::Implicit, ParametricValue::Implicit) => ParametricValue::Implicit,
        })
    }
}

impl Debug for ParametricValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(value) => f.debug_tuple("Literal").field(value).finish(),
            Self::Range(Range {
                min: start,
                max: end,
            }) => f
                .debug_tuple("Constrained")
                .field(start)
                .field(end)
                .finish(),
            Self::Implicit => write!(f, "Implicit"),
        }
    }
}
