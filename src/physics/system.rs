use physics::{FRAME_PERIOD, TIME_STEP};
use physics::body::Body;

pub struct System {
    bodies: Vec<Body>,
}

impl System {
    pub fn new() -> System {
        System {
            bodies: Vec::new(),
        }
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

    fn update_bodies(&mut self, time_step: f32) {
        for body in self.bodies.iter_mut() {
            body.update(time_step);
        }
    }
}