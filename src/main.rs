extern crate nalgebra;

#[macro_use]
extern crate glium;

mod geometry;
mod mesh;
mod scene;
mod shaders;

use glium::{glutin, DisplayBuild, Surface};
use mesh::Mesh;
use scene::Scene;
use std::rc::Rc;

fn main() {
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    let mut scene = Scene::new(&display);

    let octo = Rc::new(geometry::octohedron(&display));
    let unlit = Rc::new(shaders::unlit(&display));
    let octo_mesh = Mesh::new(octo, unlit);
    scene.add(octo_mesh);

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        scene.render(&mut target);
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }
    }
}
