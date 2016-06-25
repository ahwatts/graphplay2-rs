use nalgebra::*;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub mass: f32,
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    force: Vector3<f32>,
}

impl Body {
    pub fn new() -> Body {
        Default::default()
    }

    pub fn update(&mut self, dt: f32) {
    }
}

impl Default for Body {
    fn default() -> Body {
        Body {
            position: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),
            force: Vector3::new(0.0, 0.0, 0.0),
            mass: 0.0,
        }
    }
}
