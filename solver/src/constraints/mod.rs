pub mod point_congruence;

use crate::{
    parametric_value::ParametricValue,
    point::{Point, PointDescriptor},
};

pub struct OverconstrainedError;

pub trait PointToPointConstraint {
    fn constrain_constrainor_to_constrained(
        a: PointDescriptor,
        b: PointDescriptor,
    ) -> Result<PointDescriptor, OverconstrainedError>;
}
