use nalgebra::*;
use integrator::*;
use std::ops::{Add, AddAssign, Mul, MulAssign};

pub struct Body {
    mass: f32,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    force: Vector3<f32>,
}

impl Body {
    pub fn new() -> Body {
        Body {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            force: Vector3::new(0.0, 0.0, 0.0),
            mass: 0.0,
        }
    }

    pub fn mass(&self) -> f32 { self.mass }
    pub fn position(&self) -> Vector3<f32> { self.position }
    pub fn velocity(&self) -> Vector3<f32> { self.velocity }

    pub fn set_mass(&mut self, new_mass: f32) -> &mut Body {
        self.mass = new_mass;
        self
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) -> &mut Body {
        self.position = new_position;
        self
    }

    pub fn set_velocity(&mut self, new_velocity: Vector3<f32>) -> &mut Body {
        self.velocity = new_velocity;
        self
    }

    pub fn update(&mut self, dt: f32) {
        let new_state = {
            let state_eqn = |state: Phase, _time: f32| {
                Phase {
                    position: state.momentum / self.mass,
                    momentum: self.force / self.mass,
                }
            };

            let state = Phase {
                position: self.position,
                momentum: self.velocity * self.mass,
            };

            euler(&state_eqn, state, 0.0, dt)
        };

        self.position = new_state.position;
        self.velocity = new_state.momentum / self.mass;
        self.force = Vector3::new(0.0, 0.0, 0.0);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Phase {
    position: Vector3<f32>,
    momentum: Vector3<f32>,
}

impl Dependent<f32> for Phase {}

impl AddAssign for Phase {
    fn add_assign(&mut self, other: Phase) {
        self.position += other.position;
        self.momentum += other.momentum;
    }
}

impl MulAssign<f32> for Phase {
    fn mul_assign(&mut self, other: f32) {
        self.position *= other;
        self.momentum *= other;
    }
}

impl Add<Phase> for Phase {
    type Output = Phase;

    fn add(mut self, other: Phase) -> Phase {
        self += other;
        self
    }
}

impl Mul<f32> for Phase {
    type Output = Phase;

    fn mul(mut self, other: f32) -> Phase {
        self *= other;
        self
    }
}
