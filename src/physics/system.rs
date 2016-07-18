use nalgebra::*;
use physics::constraint::Constraint;
use physics::integrator::Dependent;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const FRAME_PERIOD: f32 = 1.0 / 60.0;
pub const TIME_STEP: f32 = 1.0 / 300.0;

pub struct System {
    id_gen: AtomicUsize,
    bodies: HashMap<BodyHandle, Body>,
    constraints: Vec<Constraint>,
}

impl System {
    pub fn new() -> System {
        System {
            id_gen: AtomicUsize::new(1),
            bodies: HashMap::new(),
            constraints: Vec::new(),
        }
    }

    pub fn create_body(&mut self) -> BodyHandle {
        let body = Body::new();
        let id = BodyHandle(self.id_gen.fetch_add(1, Ordering::Relaxed));
        self.bodies.insert(id, body);
        id
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn update(&mut self, total_time: f32) {
        let mut clamped_total_time = total_time;
        if total_time > 2.0 * FRAME_PERIOD {
            clamped_total_time = FRAME_PERIOD;
            println!("Took too long: {:?} fixing update time to {:?}", total_time, clamped_total_time);
        }

        let mut simulated_time = 0.0;
        let mut steps = 0;
        while (clamped_total_time - simulated_time) > TIME_STEP {
            self.resolve_forces();
            self.update_bodies(TIME_STEP);
            simulated_time += TIME_STEP;
            steps += 1;
        }

        self.resolve_forces();
        self.update_bodies(clamped_total_time - simulated_time);

        if steps > (FRAME_PERIOD / TIME_STEP) as usize {
            println!("Took {} steps.", steps);
        }
    }

    fn resolve_forces(&mut self) {
        // for field in self.fields.iter() {
        //     for body_cell in self.bodies.iter_mut() {
        //         let mut body = body_cell.borrow_mut();
        //         let force = field.force_on(&*body);
        //         body.add_force(force);
        //     }
        // }
    }

    fn update_bodies(&mut self, _time_step: f32) {
        // for body_cell in self.bodies.iter_mut() {
        //     let mut body = body_cell.borrow_mut();
        //     body.update(time_step);
        // }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BodyState {
    phase: Phase,
    force: Vector3<f32>,
}

const FUTURE_INDEX:  usize = 0;
const CURRENT_INDEX: usize = 1;
const OLDEST_INDEX:  usize = 1;
const KEPT_STATES:   usize = OLDEST_INDEX + 1;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BodyHandle(usize);

#[derive(Clone, Debug)]
pub struct Body(Rc<RefCell<BodyInner>>);

#[derive(Clone, Debug)]
struct BodyInner {
    asleep: bool,
    mass: f32,
    inv_mass: f32,
    states: VecDeque<BodyState>,
}

impl Body {
    fn new() -> Body {
        let initial_phase = Phase {
            position: Vector3::new(0.0, 0.0, 0.0),
            momentum: Vector3::new(0.0, 0.0, 0.0),
        };
        let initial_force = Vector3::new(0.0, 0.0, 0.0);

        let mut states = VecDeque::with_capacity(KEPT_STATES + 1);
        for _ in 0..KEPT_STATES {
            states.push_back(BodyState { phase: initial_phase, force: initial_force });
        }

        let inner = BodyInner {
            asleep: false,
            mass: 1.0,
            inv_mass: 1.0,
            states: states,
        };

        Body(Rc::new(RefCell::new(inner)))
    }

    pub fn is_asleep(&self) -> bool { self.0.borrow().asleep }
    pub fn sleep(&mut self) { self.0.borrow_mut().asleep = true }
    pub fn wake(&mut self) { self.0.borrow_mut().asleep = false }

    pub fn mass(&self) -> f32 { self.0.borrow().mass }

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
        x0*(1.0 - alpha) + x1*alpha
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) -> &mut Body {
        self.0.borrow_mut().states[FUTURE_INDEX].phase.position = new_position;
        self
    }

    pub fn velocity(&self, alpha: f32) -> Vector3<f32> {
        let inner = self.0.borrow();
        let p0 = inner.states[CURRENT_INDEX].phase.momentum;
        let p1 = inner.states[FUTURE_INDEX].phase.momentum;
        (p0*(1.0 - alpha) + p1*alpha)*inner.inv_mass
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

    pub fn update(&mut self, new_phase: Phase) {
        let mut inner = self.0.borrow_mut();
        inner.states.push_front(BodyState {
            phase: new_phase,
            force: Vector3::new(0.0, 0.0, 0.0),
        });
        inner.states.pop_back();

        // let state = inner.states[FUTURE_INDEX].phase;
        // let force = inner.states[FUTURE_INDEX].force;

        // let new_state = self.integrator.step(state, 0.0, dt, &|y: Phase, _t: f32| {
        //     // This closure returns the *derivatives* of the position
        //     // and momentum, i.e, the velocity and the net force.
        //     Phase {
        //         position: y.momentum * self.inv_mass,
        //         momentum: force,
        //     }
        // });
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
