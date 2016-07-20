use nalgebra::*;
use physics::body::Body;

#[allow(dead_code)]
pub struct Constraint {
    body_a: Body,
    body_b: Body,
    calc: Box<ConstraintCalc>,
}

pub trait ConstraintCalc {
    // Returns the force on body_a as a result of the
    // constraint. Negate to get the force on body_b.
    fn force(&self, constraint: &Constraint, pos_a: Point3<f32>, pos_b: Point3<f32>) -> Vector3<f32>;
}

pub struct Spring(pub f32);

impl Spring {
    pub fn new(spring_constant: f32) -> Spring {
        Spring(spring_constant)
    }
}

impl ConstraintCalc for Spring {
    fn force(&self, _: &Constraint, pos_a: Point3<f32>, pos_b: Point3<f32>) -> Vector3<f32> {
        let sep = pos_b - pos_a;
        sep * self.0
    }
}
