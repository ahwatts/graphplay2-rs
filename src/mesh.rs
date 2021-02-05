use std::rc::Rc;

use glium::{
    index::{Index, IndicesSource},
    vertex::VerticesSource,
    Program, Vertex,
};
use nalgebra::{Point3, Rotation3};
use num::One;

use crate::{geometry::Geometry, scene::SceneObject, shaders::ModelTransformation};

pub struct Mesh<V: Vertex, I: Index> {
    geometry: Rc<Geometry<V, I>>,
    program: Rc<Program>,

    pub position: Point3<f32>,
    pub scale: f32,
    pub orientation: Rotation3<f32>,
}

impl<V: Vertex, I: Index> Mesh<V, I> {
    pub fn new(geometry: Rc<Geometry<V, I>>, program: Rc<Program>) -> Mesh<V, I> {
        Mesh {
            geometry,
            program,
            position: Point3::origin(),
            scale: 1.0,
            orientation: Rotation3::one(),
        }
    }
}

impl<V: Vertex, I: Index> SceneObject for Mesh<V, I> {
    fn vertices(&self) -> VerticesSource {
        self.geometry.vertex_buffer().into()
    }

    fn indices(&self) -> IndicesSource {
        IndicesSource::from(self.geometry.index_buffer())
    }

    fn program(&self) -> &Program {
        &self.program
    }

    fn model_transform(&self) -> ModelTransformation {
        // let transform = Similarity3::from_parts(
        //     Translation::from(self.position.into()),
        //     self.orientation,
        //     self.scale);
        // let model = transform.to_homogeneous();

        // // Since we're only doing uniform scaling, the normal matrix
        // // (the inverse transpose of the upper-left 3x3 of the model
        // // matrix) is equal to the upper-left 3x3 of the model matrix.
        // let model_normal = FromHomogeneous::from(&model);

        ModelTransformation {
            model: One::one(),
            model_normal: One::one(),
        }
    }
}
