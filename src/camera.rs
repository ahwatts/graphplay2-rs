use nalgebra::*;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub struct Camera<F: BaseFloat + ApproxEq<F>> {
    pub position: Point3<F>,
    pub looking_at: Point3<F>,
    pub up: Vector3<F>,
}

impl<F: BaseFloat + ApproxEq<F> + Debug> Camera<F> {
    pub fn new(position: Point3<F>, focus_point: Point3<F>, up: Vector3<F>) -> Camera<F> {
        Camera {
            position: position,
            looking_at: focus_point,
            up: up,
        }
    }

    pub fn rotate(&mut self, dtheta: F, dphi: F) {
        let theta_rot: Rotation3<F> = Rotation3::new(self.up * dtheta);
        let mut cam_dir = self.position - self.looking_at;
        let cam_dist = cam_dir.norm();
        cam_dir /= cam_dist;

        cam_dir = theta_rot.rotate(&cam_dir);

        let horizontal = cross(&self.up, &cam_dir);
        let phi_rot: Rotation3<F> = Rotation3::new(horizontal * dphi);
        cam_dir = phi_rot.rotate(&cam_dir);

        let angle = dot(&cam_dir, &self.up);
        if angle > Cast::from(0.99863) || angle < Cast::from(-0.99863) {
            cam_dir = phi_rot.inverse_rotate(&cam_dir);
        }

        self.position = self.looking_at + cam_dir*cam_dist;
    }

    // pub fn zoom(&mut self, dr: F) {
    //     unimplemented!();
    // }

    pub fn view_transform(&self) -> Isometry3<F> {
        Isometry3::look_at_rh(&self.position, &self.looking_at, &self.up)
    }
}

impl<F: BaseFloat + ApproxEq<F>> Default for Camera<F> {
    fn default() -> Camera<F> {
        Camera {
            position:   Point3::<F>::origin() + Vector3::z(),
            looking_at: Point3::origin(),
            up:         Vector3::y(),
        }
    }
}
