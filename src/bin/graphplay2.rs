extern crate byteorder;
extern crate graphplay2;
extern crate nalgebra;
extern crate num;

#[macro_use]
extern crate glium;

use glium::{glutin, DisplayBuild, Surface};
use graphplay2::body::Body;
use graphplay2::camera::Camera;
use graphplay2::events::Events;
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

    let mut events: Events = Default::default();

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

    let bunny = Rc::new(geometry::load_ply(&display, "geometry/stanford_bunny.ply"));
    let bunny_mesh = Rc::new(RefCell::new(Mesh::new(bunny, lit)));
    let mut bunny_body = Body::new();
    bunny_body.set_position(Vector3 { x: 10.0, y: 0.0, z: 0.0 });
    scene.add_object(bunny_mesh.clone());

    let mut prev_time = Instant::now();

    loop {
        events.pump(&display);

        // Get the elapsed time.
        let time = Instant::now();
        let elapsed = time.duration_since(prev_time);
        prev_time = time;
        let secs = elapsed.as_secs() as f32;
        let subsecs = elapsed.subsec_nanos() as f32 / 1_000_000_000_f32;
        let ftime = secs + subsecs;

        // Update the world.
        let restoring = bunny_body.position() * -0.4;
        bunny_body.add_force(restoring);
        bunny_body.update(ftime);
        bunny_mesh.borrow_mut().position = bunny_body.position().to_point();

        // Update the camera.
        if events.left_click {
            let theta = -2.0 * f32::pi() * events.mouse_delta.x / (scene.viewport().width as f32);
            let phi   = -1.0 * f32::pi() * events.mouse_delta.y / (scene.viewport().height as f32);
            scene.camera.rotate(theta, phi);
        }

        // Render.
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        scene.render(&mut target);
        target.finish().unwrap();

        if events.quit {
            break;
        }
    }
}
