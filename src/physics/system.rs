use physics::body::Body;
use physics::{FRAME_PERIOD, TIME_STEP};

pub struct System {
    bodies: Vec<Body>,
    hangover_time: f32,
}

impl System {
    pub fn new() -> System {
        System {
            bodies: Vec::new(),
            hangover_time: 0.0,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body)
    }

    pub fn update(&mut self, frame_time: f32) -> f32 {
        // We already simulated the hangover time, so we remove it
        // from the frame time.
        let mut time_to_simulate = frame_time - self.hangover_time;

        // Prevent the spiral of death: If the app is running slowly,
        // cap the number of physics executions we're going to run in
        // a frame.
        if time_to_simulate > 10.0 * FRAME_PERIOD {
            time_to_simulate = FRAME_PERIOD;
            println!(
                "Took too long: {:?} fixing update time to {:?}",
                frame_time, time_to_simulate
            );
        }

        // Run as many steps as we can such that we've run over the
        // entire time step.
        let mut simulated_time = 0.0;
        let mut steps = 0;
        while simulated_time < time_to_simulate {
            self.resolve_forces();
            self.update_bodies(TIME_STEP);
            simulated_time += TIME_STEP;
            steps += 1;
        }

        // If we took too many steps, the app could be running slow,
        // so log it.
        if steps > 1 + (FRAME_PERIOD / TIME_STEP) as usize {
            println!("Took {} steps.", steps);
        }

        // With the fixed time steps, we simulated until less than a
        // timestep past the frame time. The time from the last full
        // step to the end of the frame is called the
        // "remainder_time", and the part that lives in the next frame
        // is called the "hangover_time".
        let remainder_time = simulated_time - time_to_simulate;
        self.hangover_time = TIME_STEP - remainder_time;
        remainder_time / TIME_STEP
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
