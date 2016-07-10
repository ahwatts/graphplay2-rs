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

    pub fn rotate(&mut self, _dtheta: F, _dphi: F) {
        // glm::vec3 y_axis(0.0, 1.0, 0.0);
        // glm::vec3 cam_dir = m_position - m_focus_point;
        // float radius = glm::length(cam_dir);
        // cam_dir = glm::normalize(cam_dir);
        
        // cam_dir = glm::rotate(cam_dir, (float)dtheta, y_axis);

        // glm::vec3 horizontal = glm::normalize(glm::cross(y_axis, cam_dir));
        // cam_dir = glm::rotate(cam_dir, (float)dphi, horizontal);

        // float angle = glm::dot(cam_dir, y_axis);
        // if (angle > 0.99863 || angle < -0.99863) {
        //     cam_dir = glm::rotate(cam_dir, (float)(-1 * dphi), horizontal);
        // }

        // m_position = m_focus_point + cam_dir*radius;

        unimplemented!();
    }

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
