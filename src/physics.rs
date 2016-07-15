use body::Body;
use nalgebra::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub static FRAME_PERIOD: f32 = 1.0 / 60.0;
pub static TIME_STEP: f32 = 1.0 / 300.0;

pub struct PhysicsSystem {
    bodies: Vec<Rc<RefCell<Body>>>,
    fields: Vec<Box<VectorField>>,
}

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem {
            bodies: vec!(),
            fields: vec!(),
        }
    }

    pub fn add_body(&mut self, body: Body) -> Weak<RefCell<Body>> {
        let body_rc = Rc::new(RefCell::new(body));
        self.bodies.push(body_rc.clone());
        Rc::downgrade(&body_rc)
    }

    pub fn add_field<F: VectorField + 'static>(&mut self, field: F) {
        self.fields.push(Box::new(field));
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
        for field in self.fields.iter() {
            for body_cell in self.bodies.iter_mut() {
                let mut body = body_cell.borrow_mut();
                let force = field.force_on(&*body);
                body.add_force(force);
            }
        }
    }

    fn update_bodies(&mut self, time_step: f32) {
        for body_cell in self.bodies.iter_mut() {
            let mut body = body_cell.borrow_mut();
            body.update(time_step);
        }
    }
}

pub trait VectorField {
    fn at_point(&self, position: &Point3<f32>) -> Vector3<f32>;
    fn force_on(&self, body: &Body) -> Vector3<f32>;
    fn force_on_at(&self, body: &Body, at: &Point3<f32>) -> Vector3<f32>;
}

pub struct Spring {
    position: Point3<f32>,
    springiness: f32,
}

impl Spring {
    pub fn new(position: Point3<f32>, spring_constant: f32) -> Spring {
        Spring {
            position: position,
            springiness: spring_constant,
        }
    }
}

impl VectorField for Spring {
    fn at_point(&self, point: &Point3<f32>) -> Vector3<f32> {
        (*point.as_vector() - *self.position.as_vector()) * self.springiness * -1.0
    }

    fn force_on(&self, body: &Body) -> Vector3<f32> {
        self.at_point(body.position().as_point())
    }

    fn force_on_at(&self, _body: &Body, point: &Point3<f32>) -> Vector3<f32> {
        self.at_point(point)
    }
}

static GRAVITATIONAL_CONSTANT: f32 = 6.67e-11;

pub struct GravitationalField {
    position: Point3<f32>,
    mass: f32,
}

impl VectorField for GravitationalField {
    fn at_point(&self, point: &Point3<f32>) -> Vector3<f32> {
        let mut r = *point.as_vector() - *self.position.as_vector();
        let r_mag2 = r.norm_squared();
        r.normalize_mut();
        let g_mag = GRAVITATIONAL_CONSTANT * self.mass / r_mag2;
        r * -1.0 * g_mag
    }

    fn force_on(&self, body: &Body) -> Vector3<f32> {
        self.force_on_at(body, body.position().as_point())
    }

    fn force_on_at(&self, body: &Body, point: &Point3<f32>) -> Vector3<f32> {
        self.at_point(point) * body.mass()
    }
}
