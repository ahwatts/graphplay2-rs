use nalgebra::*;
use physics::integrator::*;
use std::collections::VecDeque;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
struct BodyState {
    phase: Phase,
    force: Vector3<f32>,
}

const FUTURE_INDEX:  usize = 0;
const CURRENT_INDEX: usize = 1;
const OLDEST_INDEX:  usize = 2;
const KEPT_STATES:   usize = OLDEST_INDEX + 1;

pub struct Body {
    mass: f32,
    states: VecDeque<BodyState>,
    integrator: Box<Integrator<Phase, f32>>,
}

impl Body {
    pub fn new() -> Body {
        let initial_phase = Phase {
            position: Vector3::new(0.0, 0.0, 0.0),
            momentum: Vector3::new(0.0, 0.0, 0.0),
        };
        let initial_force = Vector3::new(0.0, 0.0, 0.0);

        let mut states = VecDeque::with_capacity(KEPT_STATES);
        for _ in 0..KEPT_STATES {
            states.push_back(BodyState { phase: initial_phase, force: initial_force });
        }

        Body {
            mass: 1.0,
            states: states,
            integrator: Box::new(euler),
        }
    }

    pub fn mass(&self) -> f32 { self.mass }
    pub fn position(&self) -> Vector3<f32> { self.states[CURRENT_INDEX].phase.position }
    pub fn velocity(&self) -> Vector3<f32> { self.states[CURRENT_INDEX].phase.momentum / self.mass }

    pub fn set_mass(&mut self, new_mass: f32) -> &mut Body {
        self.mass = new_mass;
        self
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) -> &mut Body {
        self.states[FUTURE_INDEX].phase.position = new_position;
        self
    }

    pub fn set_velocity(&mut self, new_velocity: Vector3<f32>) -> &mut Body {
        self.states[FUTURE_INDEX].phase.momentum = new_velocity * self.mass;
        self
    }

    pub fn net_force(&self) -> Vector3<f32> {
        self.states[CURRENT_INDEX].force
    }

    pub fn add_force(&mut self, new_force: Vector3<f32>) {
        self.states[FUTURE_INDEX].force += new_force;
    }

    pub fn update(&mut self, dt: f32) {
        let state = self.states[FUTURE_INDEX].phase;
        let force = self.states[FUTURE_INDEX].force;

        let new_state = self.integrator.step(state, 0.0, dt, &|y: Phase, _t: f32| {
            // This closure returns the *derivatives* of the position
            // and momentum, i.e, the velocity and the net force.
            Phase {
                position: y.momentum / self.mass,
                momentum: force,
            }
        });

        self.states.push_front(BodyState {
            phase: new_state,
            force: Vector3::new(0.0, 0.0, 0.0),
        });
        self.states.pop_back();
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
