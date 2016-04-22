extern crate nalgebra;

#[macro_use]
extern crate glium;

pub mod geometry;
pub mod shaders;

use glium::{glutin, Depth, DepthTest, DisplayBuild, DrawParameters, Surface};
use glium::uniforms::UniformBuffer;
use nalgebra::*;
use shaders::ViewAndProjectionBlock;

fn main() {
    let display = glutin::WindowBuilder::new().build_glium().unwrap();
    let octo = geometry::octohedron(&display);
    let unlit = shaders::unlit(&display);

    let view = Isometry3::look_at_rh(
        &Point3  { x: 0.0, y: 0.0, z: 5.0 },
        &Point3  { x: 0.0, y: 0.0, z: 0.0 },
        &Vector3 { x: 0.0, y: 1.0, z: 0.0 });
    let projection = PerspectiveMatrix3::new(4.0 / 3.0, f32::pi() / 6.0, 0.1, 100.0);
    let model = one::<Matrix4<f32>>();
    let model_inv_trans_3 = one::<Matrix3<f32>>();

    let vp_block = ViewAndProjectionBlock {
        view: *view.to_homogeneous().as_ref(),
        view_inv: *view.inverse_transformation().to_homogeneous().as_ref(),
        projection: *projection.as_matrix().as_ref(),
    };

    let vp_buffer = UniformBuffer::new(&display, vp_block).unwrap();

    let uniforms = uniform!{
        model: *model.as_ref(),
        model_inv_trans_3: *model_inv_trans_3.as_ref(),
        view_and_projection: &vp_buffer,
    };

    let draw_params = DrawParameters {
        depth: Depth {
            write: true,
            test: DepthTest::IfLess,
            .. Default::default()
        },
        .. Default::default()
    };

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        octo.render(&mut target, &unlit, &uniforms, &draw_params);
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Escape)) => return,
                // glutin::Event::KeyboardInput(element_state, scan_code, opt_vk_code) => {
                //     println!("Keyboard event: element_state = {:?} scan_code = {:?} opt_vk_code = {:?}",
                //              element_state, scan_code, opt_vk_code);
                // },
                _ => {},
            }
        }
    }
}
