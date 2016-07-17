use nalgebra::*;

pub use self::body::Body;
pub use self::integrator::{euler, rk4, Independent, Dependent, FirstOrderODE, Integrator};
pub use self::system::System;

pub mod body;
pub mod integrator;
pub mod system;

pub static FRAME_PERIOD: f32 = 1.0 / 60.0;
pub static TIME_STEP: f32 = 1.0 / 300.0;

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
