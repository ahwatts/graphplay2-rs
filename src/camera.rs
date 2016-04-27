use nalgebra::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Camera<F: BaseFloat> {
    pub position: Point3<F>,
    pub looking_at: Point3<F>,
    pub up: Vector3<F>,
}

impl<F: BaseFloat> Camera<F> {
    pub fn new(position: Point3<F>, focus_point: Point3<F>, up: Vector3<F>) -> Camera<F> {
        Camera {
            position: position,
            looking_at: focus_point,
            up: up,
        }
    }

    // pub fn rotate(&mut self, dtheta: F, dphi: F) {
    //     unimplemented!();
    // }

    // pub fn zoom(&mut self, dr: F) {
    //     unimplemented!();
    // }

    pub fn view_transform(&self) -> Isometry3<F> {
        Isometry3::look_at_rh(&self.position, &self.looking_at, &self.up)
    }
}

impl<F: BaseFloat> Default for Camera<F> {
    fn default() -> Camera<F> {
        Camera {
            position:   Point3::<F>::origin() + Vector3::z(),
            looking_at: Point3::origin(),
            up:         Vector3::y(),
        }
    }
}
