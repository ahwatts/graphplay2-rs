use camera::Camera;
use glium::backend::Facade;
use glium::draw_parameters::{DrawParameters, Depth, DepthTest};
use glium::vertex::VerticesSource;
use glium::index::IndicesSource;
use glium::program::Program;
use glium::uniforms::UniformBuffer;
use glium::{Rect, Surface};
use nalgebra::*;
use shaders::{LightListBlock, LightProperties, ModelTransformation, ViewAndProjectionBlock};
use std::cell::RefCell;
use std::rc::Rc;

pub trait SceneObject {
    fn vertices(&self) -> VerticesSource;
    fn indices(&self) -> IndicesSource;
    fn program(&self) -> &Program;
    fn model_transform(&self) -> ModelTransformation;
}

pub struct Scene {
    objects: Vec<Rc<RefCell<SceneObject>>>,
    // camera: Camera<f32>,
    viewport: Rect,

    vp_buffer: UniformBuffer<ViewAndProjectionBlock>,
    light_buffer: UniformBuffer<LightListBlock>,
}

impl Scene {
    pub fn new<F: Facade>(display: &F, camera: Camera<f32>, width: u32, height: u32) -> Scene {
        let view = camera.view_transform();
        let projection = PerspectiveMatrix3::new(
            width as f32 / height as f32,
            f32::pi() / 6.0,
            0.1, 100.0);

        let vp_block = ViewAndProjectionBlock {
            view: *view.to_homogeneous().as_ref(),
            view_inv: *view.inverse_transformation().to_homogeneous().as_ref(),
            projection: *projection.as_matrix().as_ref(),
        };

        let mut light_list_block: LightListBlock = Default::default();
        light_list_block.lights[0] = LightProperties::new(true, [ 0.0, 10.0, 10.0 ], [ 1.0, 1.0, 1.0, 1.0 ], 10.0);

        Scene {
            objects: Vec::new(),
            // camera: camera,
            viewport: Rect { left: 0, bottom: 0, width: width, height: height },

            vp_buffer: UniformBuffer::new(display, vp_block).unwrap(),
            light_buffer: UniformBuffer::new(display, light_list_block).unwrap(),
        }
    }

    pub fn add<O: SceneObject + 'static>(&mut self, object: Rc<RefCell<O>>) {
        self.objects.push(object);
    }

    pub fn render<S: Surface>(&self, target: &mut S) {
        let draw_params = DrawParameters {
            depth: Depth {
                write: true,
                test: DepthTest::IfLess,
                .. Default::default()
            },
            viewport: Some(self.viewport),
            .. Default::default()
        };

        for object_cell in self.objects.iter() {
            let object = object_cell.borrow();
            let model_transform = object.model_transform();

            let uniforms = uniform! {
                model: *model_transform.model.as_ref(),
                model_normal: *model_transform.model_normal.as_ref(),
                view_and_projection: &self.vp_buffer,
                light_list: &self.light_buffer,
            };

            target.draw(object.vertices(), object.indices(),
                        object.program(), &uniforms,
                        &draw_params)
                .unwrap();
        }
    }
}
