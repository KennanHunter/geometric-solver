use crate::{
    parametric_value::ParametricValue,
    point::{Point, PointDescriptor},
};

use super::{OverconstrainedError, PointToPointConstraint};

struct PointCongruence(Point, Point);

impl PointToPointConstraint for PointCongruence {
    fn constrain_constrainor_to_constrained(
        constrained: PointDescriptor,
        constrainor: PointDescriptor,
    ) -> Result<PointDescriptor, OverconstrainedError> {
        let x = ParametricValue::choose_most_specific(constrained.x, constrainor.x)?;
        let y = ParametricValue::choose_most_specific(constrained.y, constrainor.y)?;

        Ok(PointDescriptor { x, y })
    }
}
