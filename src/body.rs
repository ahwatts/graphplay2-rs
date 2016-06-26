use nalgebra::*;
use integrator::*;
use std::cell::RefCell;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Body {
    pub mass: f32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    force: Vector3<f32>,
    integrator: Option<Box<Integrator<Phase, f32>>>,
}

impl Body {
    pub fn new() -> Rc<RefCell<Body>> {
        let body = Rc::new(RefCell::new(Body {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            force: Vector3::new(0.0, 0.0, 0.0),
            mass: 0.0,
            integrator: None,
        }));

        let equation = Box::new(BodyStateEquation {
            body: Rc::downgrade(&body),
        });

        let init_phase = Phase {
            position: Vector3::new(0.0, 0.0, 0.0),
            momentum: Vector3::new(0.0, 0.0, 0.0),
        };

        let init_time = 0.0;

        body.borrow_mut().integrator = Some(Box::new(Euler::new(equation, init_phase, init_time)));

        body
    }

    pub fn update(&mut self, dt: f32) {
        self.integrator.as_mut().unwrap().step(&dt);
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

#[derive(Clone, Debug)]
pub struct BodyStateEquation {
    body: Weak<RefCell<Body>>,
}

impl FirstOrderODE<Phase, f32> for BodyStateEquation {
    fn derivative(&self, _pos: &Phase, _time: &f32) -> Phase {
        let strong_body = self.body.upgrade().unwrap();
        let body = strong_body.borrow();
        Phase {
            position: body.velocity,
            momentum: body.force / body.mass,
        }
    }
}
