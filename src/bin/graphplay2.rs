extern crate byteorder;
extern crate graphplay2;
extern crate nalgebra;
extern crate num;

#[macro_use]
extern crate glium;

use glium::{glutin, DisplayBuild, Surface};
use graphplay2::body::Body;
use graphplay2::camera::Camera;
use graphplay2::mesh::Mesh;
use graphplay2::scene::Scene;
use graphplay2::shaders::{self, LightProperties};
use graphplay2::geometry;
use nalgebra::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    let (width, height) = (1024, 768);

    let display = glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_depth_buffer(24)
        .build_glium()
        .unwrap();

    let (window_width, window_height) = display.get_window().unwrap()
        .get_inner_size_pixels().unwrap();

    let _unlit = Rc::new(shaders::unlit(&display));
    let lit = Rc::new(shaders::lit(&display));

    let mut scene = Scene::new(
        &display,
        Camera::new(Point3  { x: 0.0, y: 0.0, z: 50.0 },
                    Point3  { x: 0.0, y: 0.0, z:  0.0 },
                    Vector3 { x: 0.0, y: 1.0, z:  0.0 }),
        window_width, window_height);

    scene.set_light(0, LightProperties::new(true, [ 0.0, 10.0, 10.0 ], [ 1.0, 1.0, 1.0, 1.0 ], 10.0));

    // let octo = Rc::new(geometry::octohedron(&display));
    // let octo_mesh = Rc::new(RefCell::new(Mesh::new(octo, lit)));
    // scene.add_object(octo_mesh.clone());

    let bunny = Rc::new(geometry::load_ply(&display, "geometry/stanford_bunny.ply"));
    let bunny_mesh = Rc::new(RefCell::new(Mesh::new(bunny, lit)));
    let mut bunny_body = Body::new();
    bunny_body.set_position(Vector3 { x: 10.0, y: 0.0, z: 0.0 });
    scene.add_object(bunny_mesh.clone());

    // let pi = f32::pi();
    // let (mut xrot, mut yrot) = (0.0, 0.0);
    let mut prev_time = Instant::now();

    loop {
        // Get the elapsed time.
        let time = Instant::now();
        let elapsed = time.duration_since(prev_time);
        prev_time = time;
        let secs = elapsed.as_secs() as f32;
        let subsecs = elapsed.subsec_nanos() as f32 / 1_000_000_000_f32;
        let ftime = secs + subsecs;

        let restoring = bunny_body.position() * -0.2;
        bunny_body.add_force(restoring);
        bunny_body.update(ftime);
        bunny_mesh.borrow_mut().position = bunny_body.position().to_point();

        // // Update things.
        // yrot = (yrot + ftime * pi / 20.0) % (2.0 * pi);
        // xrot = (xrot + ftime * pi / 60.0) % (2.0 * pi);
        // let yrot_m = Vector3::y() * yrot;
        // let xrot_m = Vector3::x() * xrot;
        // // octo_mesh.borrow_mut().orientation = Rotation3::new(yrot_m).append_rotation(&xrot_m);
        // bunny_mesh.borrow_mut().orientation = Rotation3::new(yrot_m).append_rotation(&xrot_m);

        // Render.
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        scene.render(&mut target);
        target.finish().unwrap();

        // Handle updates.
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }
    }
}
