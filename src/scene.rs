use glium::backend::Facade;
use glium::draw_parameters::{DrawParameters, Depth, DepthTest};
use glium::vertex::VerticesSource;
use glium::index::IndicesSource;
use glium::program::Program;
use glium::uniforms::{UniformBuffer};
use glium::Surface;
use nalgebra::*;
use shaders::ViewAndProjectionBlock;
use std::cell::RefCell;
use std::rc::Rc;

pub trait SceneObject {
    fn vertices(&self) -> VerticesSource;
    fn indices(&self) -> IndicesSource;
    fn program(&self) -> &Program;
    fn model_transform(&self) -> (Matrix4<f32>, Matrix3<f32>);
}

pub struct Scene {
    objects: Vec<Rc<RefCell<SceneObject>>>,
    vp_buffer: UniformBuffer<ViewAndProjectionBlock>,
}

impl Scene {
    pub fn new<F: Facade>(display: &F) -> Scene {
        let view = Isometry3::look_at_rh(
            &Point3  { x: 0.0, y: 0.0, z: 5.0 },
            &Point3  { x: 0.0, y: 0.0, z: 0.0 },
            &Vector3 { x: 0.0, y: 1.0, z: 0.0 });
        let projection = PerspectiveMatrix3::new(
            4.0 / 3.0,
            f32::pi() / 6.0,
            0.1, 100.0);

        let vp_block = ViewAndProjectionBlock {
            view: *view.to_homogeneous().as_ref(),
            view_inv: *view.inverse_transformation().to_homogeneous().as_ref(),
            projection: *projection.as_matrix().as_ref(),
        };

        Scene {
            objects: Vec::new(),
            vp_buffer: UniformBuffer::new(display, vp_block).unwrap(),
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
            ..Default::default()
        };

        for object_cell in self.objects.iter() {
            let object = object_cell.borrow();
            let (model, model_inv_trans_3) = object.model_transform();

            let uniforms = uniform! {
                model: *model.as_ref(),
                model_inv_trans_3: *model_inv_trans_3.as_ref(),
                view_and_projection: &self.vp_buffer,
            };

            target.draw(object.vertices(), object.indices(),
                        object.program(), &uniforms,
                        &draw_params)
                .unwrap();
        }
    }
}
