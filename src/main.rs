extern crate byteorder;
extern crate nalgebra;
extern crate num;

#[macro_use]
extern crate glium;

use glium::{glutin, DisplayBuild, Surface};
use camera::Camera;
use events::Events;
use mesh::Mesh;
use physics::{Spring, Body, System};
use scene::Scene;
use shaders::LightProperties;
use nalgebra::*;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::thread;
use std::time::{Duration, Instant};

pub mod camera;
pub mod events;
pub mod geometry;
pub mod mesh;
pub mod physics;
pub mod ply;
pub mod scene;
pub mod shaders;

fn main() {
    let (width, height) = (1024, 768);
    let mut events: Events = Default::default();

    // Create the window / OpenGL context.
    let display = glutin::WindowBuilder::new()
        .with_dimensions(width, height)
        .with_title("graphplay2")
        .with_depth_buffer(24)
        .with_vsync()
        .build_glium()
        .unwrap();
    let (window_width, window_height) = display.get_window().unwrap()
        .get_inner_size_pixels().unwrap();

    // Create the shader programs.
    let unlit = Rc::new(shaders::unlit(&display));
    let lit = Rc::new(shaders::lit(&display));

    // Create the scene graph.
    let mut scene = Scene::new(
        &display,
        Camera::new(Point3  { x: 0.0, y: 0.0, z: 70.0 },
                    Point3  { x: 0.0, y: 0.0, z:  0.0 },
                    Vector3 { x: 0.0, y: 1.0, z:  0.0 }),
        window_width, window_height);
    scene.set_light(0, LightProperties::new(true, [ 0.0, 10.0, 10.0 ], [ 1.0, 1.0, 1.0, 1.0 ], 10.0));

    // Bunny.
    let bunny = Rc::new(geometry::load_ply(&display, "geometry/stanford_bunny.ply"));
    let bunny_mesh = Rc::new(RefCell::new(Mesh::new(bunny, lit)));
    scene.add_object(bunny_mesh.clone());

    // Cube boundary.
    let cube = Rc::new(geometry::wireframe_cube(&display));
    let cube_mesh = Rc::new(RefCell::new(Mesh::new(cube, unlit)));
    cube_mesh.borrow_mut().scale = 10.0;
    scene.add_object(cube_mesh.clone());

    // Create the physics environment.
    let mut world = System::new();

    let weak_bunny_body_cell = world.add_body(Body::new());
    if let Some(bunny_body_cell) = Weak::upgrade(&weak_bunny_body_cell) {
        let mut bunny_body = bunny_body_cell.borrow_mut();
        bunny_body.set_position(Vector3 { x: 10.0, y: 0.0, z: 0.0 });
    }

    world.add_field(Spring::new(Point3::origin(), 0.4));

    // Misc. loop variables.
    let mut prev_time = Instant::now();
    let pi = f32::pi();
    let frame_period = Duration::new(0, (physics::FRAME_PERIOD * 1.0e9) as u32);

    loop {
        events.pump(&display);

        // Get the elapsed time.
        let time = Instant::now();
        let elapsed = time.duration_since(prev_time);
        prev_time = time;
        let secs = elapsed.as_secs() as f32;
        let subsecs = elapsed.subsec_nanos() as f32 / 1.0e9;
        let ftime = secs + subsecs;

        // Update the world.
        world.update(ftime);
        if let Some(bunny_body_cell) = Weak::upgrade(&weak_bunny_body_cell) {
            let bunny_body = bunny_body_cell.borrow();
            bunny_mesh.borrow_mut().position = bunny_body.position().to_point();
        }

        // Update the camera.
        if events.left_click {
            let theta = -2.0 * pi * events.mouse_delta.x / (scene.viewport().width as f32);
            let phi   = -1.0 * pi * events.mouse_delta.y / (scene.viewport().height as f32);
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

        let update_time = Instant::now();
        let update_duration = update_time.duration_since(time);
        // println!("Update took {:?}", update_duration);
        if update_duration < frame_period {
            let sleep_duration = frame_period - update_duration;
            // println!("Sleeping for {:?}", sleep_duration);
            thread::sleep(sleep_duration);
        }
    }
}
