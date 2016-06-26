use nalgebra::*;
use integrator::*;
use std::cell::RefCell;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::rc::{Rc, Weak};

pub struct Body(Rc<RefCell<BodyInner>>);

impl Body {
    pub fn new() -> Body {
        let inner = BodyInner::new();
        let init_phase = Phase {
            position: inner.position,
            momentum: inner.velocity * inner.mass,
        };
        let init_time = 0.0;

        let body = Body(Rc::new(RefCell::new(inner)));
        let equation = Box::new(BodyStateEquation::for_body(&body));
        // let integrator = Box::new(Euler::new(equation, init_phase, init_time));

        // body.0.borrow_mut().integrator = Some(integrator);

        body
    }

    pub fn mass(&self) -> f32 { self.0.borrow().mass }
    pub fn position(&self) -> Vector3<f32> { self.0.borrow().position }
    pub fn velocity(&self) -> Vector3<f32> { self.0.borrow().position }

    pub fn set_mass(&mut self, new_mass: f32) -> &mut Body {
        self.with_mut_inner(|inner| {
            inner.mass = new_mass;
            inner.update_integrator();
        });
        self
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>) -> &mut Body {
        self.with_mut_inner(|inner| {
            inner.position = new_position;
            inner.update_integrator();
        });
        self
    }

    pub fn set_velocity(&mut self, new_velocity: Vector3<f32>) -> &mut Body {
        self.with_mut_inner(|inner| {
            inner.velocity = new_velocity;
            inner.update_integrator();
        });
        self
    }

    pub fn update(&mut self, dt: f32) {
        self.with_mut_inner(|inner| {
            // inner.integrator.as_mut().unwrap().step(&dt);
        });
    }

    fn with_mut_inner<F, R>(&mut self, func: F) -> R
        where F: Fn(&mut BodyInner) -> R
    {
        let mut inner = self.0.borrow_mut();
        func(&mut inner)
    }
}

#[derive(Debug)]
struct BodyInner {
    mass: f32,
    position: Vector3<f32>,
    velocity: Vector3<f32>,
    force: Vector3<f32>,
    // integrator: Option<Box<Integrator<Phase, f32>>>,
}

impl BodyInner {
    fn new() -> BodyInner {
        BodyInner {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            force: Vector3::new(0.0, 0.0, 0.0),
            mass: 0.0,
            // integrator: None,
        }
    }

    fn update_integrator(&mut self) {
        let phase = Phase {
            position: self.position,
            momentum: self.velocity * self.mass,
        };
        // self.integrator.as_mut().unwrap()
        //     .set_dependent(phase);
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
    body: Weak<RefCell<BodyInner>>,
}

impl BodyStateEquation {
    pub fn for_body(body: &Body) -> BodyStateEquation {
        BodyStateEquation {
            body: Rc::downgrade(&body.0),
        }
    }
}

impl FirstOrderODE<Phase, f32> for BodyStateEquation {
    fn derivative(&self, _pos: Phase, _time: f32) -> Phase {
        let strong_body = self.body.upgrade().unwrap();
        let body = strong_body.borrow();
        Phase {
            position: body.velocity,
            momentum: body.force / body.mass,
        }
    }
}
