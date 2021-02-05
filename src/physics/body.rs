use std::{
    cell::RefCell,
    collections::VecDeque,
    fmt::{self, Debug},
    ops::{Add, AddAssign, Mul, MulAssign},
    rc::Rc,
};

use nalgebra::Vector3;

use super::{constraint::ConstraintCalc, euler, Constraint, Dependent, Integrator};

#[derive(Clone, Copy, Debug, PartialEq)]
struct BodyState {
    phase: Phase,
    force: Vector3<f32>,
}

const FUTURE_INDEX: usize = 0;
const CURRENT_INDEX: usize = 1;
const OLDEST_INDEX: usize = 1;
const KEPT_STATES: usize = OLDEST_INDEX + 1;

#[derive(Clone, Debug)]
pub struct Body(Rc<RefCell<BodyInner>>);

struct BodyInner {
    asleep: bool,
    mass: f32,
    inv_mass: f32,
    states: VecDeque<BodyState>,
    integrator: Box<dyn Integrator<Phase, f32>>,
    constraints: Vec<Constraint>,
}

impl Body {
    pub fn new() -> Body {
        let initial_phase = Phase {
            position: Vector3::new(0.0, 0.0, 0.0),
            momentum: Vector3::new(0.0, 0.0, 0.0),
        };
        let initial_force = Vector3::new(0.0, 0.0, 0.0);

        let mut states = VecDeque::with_capacity(KEPT_STATES + 1);
        for _ in 0..KEPT_STATES {
            states.push_back(BodyState {
                phase: initial_phase,
                force: initial_force,
            });
        }

        let inner = BodyInner {
            asleep: false,
            mass: 1.0,
            inv_mass: 1.0,
            states,
            integrator: Box::new(euler),
            constraints: Vec::new(),
        };

        Body(Rc::new(RefCell::new(inner)))
    }

    pub fn add_constraint(&mut self, calc: Rc<dyn ConstraintCalc>, other: Body) {
        self.0.borrow_mut().constraints.push(Constraint {
            other: other.clone(),
            calc: calc.clone(),
        });
    }

    pub fn is_asleep(&self) -> bool {
        self.0.borrow().asleep
    }
    pub fn sleep(&mut self) {
        self.0.borrow_mut().asleep = true
    }
    pub fn wake(&mut self) {
        self.0.borrow_mut().asleep = false
    }

    pub fn mass(&self) -> f32 {
        self.0.borrow().mass
    }

    pub fn set_mass(&mut self, new_mass: f32) -> &mut Body {
        {
            let mut inner = self.0.borrow_mut();
            inner.mass = new_mass;
            inner.inv_mass = 1.0 / new_mass;
        }
        self
    }

    pub fn position(&self, alpha: f32) -> Vector3<f32> {
        let inner = self.0.borrow();
        let x0 = inner.states[CURRENT_INDEX].phase.position;
        let x1 = inner.states[FUTURE_INDEX].phase.position;
        x0 * (1.0 - alpha) + x1 * alpha
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) -> &mut Body {
        self.0.borrow_mut().states[FUTURE_INDEX].phase.position = new_position;
        self
    }

    pub fn velocity(&self, alpha: f32) -> Vector3<f32> {
        let inner = self.0.borrow();
        let p0 = inner.states[CURRENT_INDEX].phase.momentum;
        let p1 = inner.states[FUTURE_INDEX].phase.momentum;
        (p0 * (1.0 - alpha) + p1 * alpha) * inner.inv_mass
    }

    pub fn set_velocity(&mut self, new_velocity: Vector3<f32>) -> &mut Body {
        {
            let mut inner = self.0.borrow_mut();
            inner.states[FUTURE_INDEX].phase.momentum = new_velocity * inner.mass;
        }
        self
    }

    pub fn net_force(&self) -> Vector3<f32> {
        self.0.borrow().states[CURRENT_INDEX].force
    }

    pub fn add_force(&mut self, new_force: Vector3<f32>) {
        self.0.borrow_mut().states[FUTURE_INDEX].force += new_force;
    }

    pub fn update(&mut self, time_step: f32) {
        let mut inner = self.0.borrow_mut();

        let phase = inner.states[FUTURE_INDEX].phase;
        let base_force = inner.states[FUTURE_INDEX].force;

        let new_phase = inner
            .integrator
            .step(phase, 0.0, time_step, &|y: Phase, _t: f32| {
                let mut total_force = base_force;
                for c in inner.constraints.iter() {
                    total_force += c.calc.force(&y.position, &c.other.position(1.0));
                }

                // This closure returns the *derivatives* of the position
                // and momentum, i.e, the velocity and the net force.
                Phase {
                    position: y.momentum * inner.inv_mass,
                    momentum: total_force,
                }
            });

        inner.states.push_front(BodyState {
            phase: new_phase,
            force: Vector3::new(0.0, 0.0, 0.0),
        });
        inner.states.pop_back();
    }
}

impl Debug for BodyInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Body")
            .field("asleep", &self.asleep)
            .field("mass", &self.mass)
            .field("inv_mass", &self.inv_mass)
            .field("states", &self.states)
            .finish()
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
