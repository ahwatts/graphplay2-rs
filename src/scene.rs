use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use glium::{
    backend::Facade, index::IndicesSource, uniforms::UniformBuffer, vertex::VerticesSource, Depth,
    DepthTest, DrawParameters, Program, Rect, Surface,
};
use nalgebra::Perspective3;

use crate::{
    camera::Camera,
    shaders::{LightListBlock, LightProperties, ModelTransformation, ViewAndProjectionBlock},
};

pub trait SceneObject {
    fn vertices(&self) -> VerticesSource;
    fn indices(&self) -> IndicesSource;
    fn program(&self) -> &Program;
    fn model_transform(&self) -> ModelTransformation;
}

pub struct Scene {
    objects: Vec<Rc<RefCell<dyn SceneObject>>>,
    pub camera: Camera<f32>,
    viewport: Rect,

    vp_buffer: UniformBuffer<ViewAndProjectionBlock>,
    light_buffer: UniformBuffer<LightListBlock>,
}

impl Scene {
    pub fn new<F: Facade>(display: &F, camera: Camera<f32>, width: u32, height: u32) -> Scene {
        let view = camera.view_transform();
        let projection = Perspective3::new(width as f32 / height as f32, PI / 6.0, 0.1, 100.0);

        let vp_block = ViewAndProjectionBlock {
            view: *view.to_homogeneous().as_ref(),
            view_inv: *view.inverse().to_homogeneous().as_ref(),
            projection: *projection.as_matrix().as_ref(),
        };

        Scene {
            objects: Vec::new(),
            camera,
            viewport: Rect {
                left: 0,
                bottom: 0,
                width,
                height,
            },

            vp_buffer: UniformBuffer::new(display, vp_block).unwrap(),
            light_buffer: UniformBuffer::dynamic(display, Default::default()).unwrap(),
        }
    }

    pub fn viewport(&self) -> Rect {
        self.viewport
    }

    pub fn add_object<O: SceneObject + 'static>(&mut self, object: Rc<RefCell<O>>) {
        self.objects.push(object);
    }

    pub fn set_light(&mut self, index: usize, light: LightProperties) {
        let mut mapped_lights = self.light_buffer.map();
        mapped_lights.lights[index] = light;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.viewport.width = width;
        self.viewport.height = height;
        let projection = Perspective3::new(width as f32 / height as f32, PI / 6.0, 0.1, 100.0);
        let mut mapped_vp = self.vp_buffer.map();
        mapped_vp.projection = *projection.as_matrix().as_ref();
    }

    fn update_view(&mut self) {
        let view = self.camera.view_transform();
        let mut mapped_vp = self.vp_buffer.map();
        mapped_vp.view = *view.to_homogeneous().as_ref();
        mapped_vp.view_inv = *view.inverse().to_homogeneous().as_ref();
    }

    pub fn render<S: Surface>(&mut self, target: &mut S) {
        let draw_params = DrawParameters {
            depth: Depth {
                write: true,
                test: DepthTest::IfLess,
                ..Default::default()
            },
            viewport: Some(self.viewport),
            ..Default::default()
        };

        self.update_view();

        for object_cell in self.objects.iter() {
            let object = object_cell.borrow();
            let model_transform = object.model_transform();

            let uniforms = uniform! {
                model: *model_transform.model.as_ref(),
                model_normal: *model_transform.model_normal.as_ref(),
                view_and_projection: &self.vp_buffer,
                light_list: &self.light_buffer,
            };

            target
                .draw(
                    object.vertices(),
                    object.indices(),
                    object.program(),
                    &uniforms,
                    &draw_params,
                )
                .unwrap();
        }
    }
}
