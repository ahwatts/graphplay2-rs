use std::{
    cell::RefCell,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use camera::Camera;
use glium::{
    glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display, Surface,
};
use mesh::Mesh;
use nalgebra::{Point3, Vector3};
use physics::{Body, Spring, System, FRAME_PERIOD};
use scene::Scene;
use shaders::LightProperties;

extern crate byteorder;
extern crate nalgebra;
extern crate num;

#[macro_use]
extern crate glium;

pub mod camera;
// pub mod events;
pub mod geometry;
pub mod mesh;
pub mod physics;
pub mod ply;
pub mod scene;
pub mod shaders;

fn main() {
    let (width, height) = (1024, 768);

    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(width, height))
        .with_title("graphplay2");
    let context_builder = ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true)
        .with_double_buffer(Some(true));
    let display =
        Display::new(window_builder, context_builder, &event_loop).expect("Error creating display");

    let (window_width, window_height) = display.gl_window().window().inner_size().into();

    // Create the shader programs.
    let unlit = Rc::new(shaders::unlit(&display));
    let lit = Rc::new(shaders::lit(&display));

    // Create the scene graph.
    let mut scene = Scene::new(
        &display,
        Camera::new(
            Point3::new(0.0, 0.0, 70.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        ),
        window_width,
        window_height,
    );
    scene.set_light(
        0,
        LightProperties::new(true, [0.0, 10.0, 10.0], [1.0, 1.0, 1.0, 1.0], 10.0),
    );

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

    let mut bunny = Body::new();
    let origin = Body::new();
    bunny.set_position(Vector3::new(10.0, 0.0, 0.0));
    bunny.add_constraint(Rc::new(Spring(5.0)), origin);
    world.add_body(bunny.clone());

    // Misc. loop variables.
    let mut prev_time = Instant::now();
    let frame_period = Duration::new(0, (FRAME_PERIOD * 1.0e9) as u32);

    let mut frame_count = 0;
    let mut avg_update_secs = 0.0;
    let mut avg_sleep_secs = 0.0;
    let mut avg_real_sleep_secs = 0.0;

    loop {
        // events.pump(&display);

        // Get the elapsed time.
        let time = Instant::now();
        let elapsed = time.duration_since(prev_time);
        prev_time = time;
        let secs = elapsed.as_secs() as f32;
        let subsecs = elapsed.subsec_nanos() as f32 / 1.0e9;
        let ftime = secs + subsecs;

        // Update the world.
        let step_fraction = world.update(ftime);
        bunny_mesh.borrow_mut().position = bunny.position(step_fraction).into();

        // Update the camera.
        // if events.left_click {
        //     let theta = -2.0 * PI * events.mouse_delta.x / (scene.viewport().width as f32);
        //     let phi   = -1.0 * PI * events.mouse_delta.y / (scene.viewport().height as f32);
        //     scene.camera.rotate(theta, phi);
        // }

        // Render.
        let mut target = display.draw();
        target.clear(None, Some((0.0, 0.0, 0.0, 1.0)), true, Some(1.0), None);
        scene.render(&mut target);
        target.finish().unwrap();

        // if events.quit {
        //     break;
        // }

        let update_time = Instant::now();
        let update_duration = update_time.duration_since(time);

        frame_count += 1;
        let update_secs =
            (update_duration.as_secs() as f32) + (update_duration.subsec_nanos() as f32 / 1.0e9);
        avg_update_secs = avg_update_secs + (update_secs - avg_update_secs) / (frame_count as f32);

        if update_duration < frame_period {
            let sleep_duration = frame_period - update_duration;
            let sleep_secs =
                (sleep_duration.as_secs() as f32) + (sleep_duration.subsec_nanos() as f32 / 1.0e9);
            avg_sleep_secs = avg_sleep_secs + (sleep_secs - avg_sleep_secs) / (frame_count as f32);

            thread::sleep(sleep_duration);

            let real_sleep_time = Instant::now();
            let real_sleep_duration = real_sleep_time - update_time;
            let real_sleep_secs = (real_sleep_duration.as_secs() as f32)
                + (real_sleep_duration.subsec_nanos() as f32 / 1.0e9);
            avg_real_sleep_secs = avg_real_sleep_secs
                + (real_sleep_secs - avg_real_sleep_secs) / (frame_count as f32);
        }
    }

    // println!("frames = {}", frame_count);
    // println!("average update time: {} ms", avg_update_secs * 1e3);
    // println!("average sleep time: {} ms", avg_sleep_secs * 1e3);
    // println!(
    //     "average actual sleep time: {} ms",
    //     avg_real_sleep_secs * 1e3
    // );
}
