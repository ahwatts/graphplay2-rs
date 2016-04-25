extern crate nalgebra;

#[macro_use]
extern crate glium;

mod geometry;
mod mesh;
mod scene;
mod shaders;

use glium::{glutin, DisplayBuild, Surface};
use mesh::Mesh;
use nalgebra::*;
use scene::Scene;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    // let unlit = Rc::new(shaders::unlit(&display));
    let lit = Rc::new(shaders::lit(&display));

    let mut scene = Scene::new(&display);

    let octo = Rc::new(geometry::octohedron(&display));
    let octo_mesh = Rc::new(RefCell::new(Mesh::new(octo, lit)));
    scene.add(octo_mesh.clone());

    let pi = f32::pi();
    let (mut xrot, mut yrot) = (0.0, 0.0);
    let mut prev_time = Instant::now();

    loop {
        // Get the elapsed time.
        let time = Instant::now();
        let elapsed = time.duration_since(prev_time);
        prev_time = time;
        let nanos = elapsed.subsec_nanos(); // Assume that the frame took < 1s.
        let ftime = nanos as f32 / 1_000_000_000_f32;

        // Build the model transformation matrix.
        yrot = (yrot + ftime * pi / 20.0) % (2.0 * pi);
        xrot = (xrot + ftime * pi / 60.0) % (2.0 * pi);
        let yrot_m = Vector3::y() * yrot;
        let xrot_m = Vector3::x() * xrot;
        octo_mesh.borrow_mut().orientation = Rotation3::new(yrot_m).append_rotation(&xrot_m);

        // Render.
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        scene.render(&mut target);
        target.finish().unwrap();

        // Handle updates.
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }
    }
}
