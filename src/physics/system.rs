use physics::VectorField;
use physics::body::Body;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub static FRAME_PERIOD: f32 = 1.0 / 60.0;
pub static TIME_STEP: f32 = 1.0 / 300.0;

pub struct System {
    bodies: Vec<Rc<RefCell<Body>>>,
    fields: Vec<Box<VectorField>>,
}

impl System {
    pub fn new() -> System {
        System {
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
