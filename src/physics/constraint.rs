use std::{fmt::Debug, rc::Rc};

use nalgebra::Vector3;

use super::Body;

#[derive(Clone, Debug)]
pub struct Constraint {
    pub other: Body,
    pub calc: Rc<dyn ConstraintCalc>,
}

pub trait ConstraintCalc: Debug {
    // Returns the force on this body as a result of the constraint
    // from other.
    fn force(&self, this_pos: &Vector3<f32>, other_pos: &Vector3<f32>) -> Vector3<f32>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Spring(pub f32);

impl Spring {
    pub fn new(spring_constant: f32) -> Spring {
        Spring(spring_constant)
    }
}

impl ConstraintCalc for Spring {
    fn force(&self, this_pos: &Vector3<f32>, other_pos: &Vector3<f32>) -> Vector3<f32> {
        let sep = *other_pos - *this_pos;
        sep * self.0
    }
}
