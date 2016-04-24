use geometry::Geometry;
use glium::index::{Index, IndicesSource};
use glium::program::Program;
use glium::vertex::{IntoVerticesSource, Vertex, VerticesSource};
use nalgebra::*;
use scene::SceneObject;
use std::rc::Rc;

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
            geometry: geometry,
            program: program,
            position: origin(),
            scale: 1.0,
            orientation: one(),
        }
    }
}

impl<V: Vertex, I: Index> SceneObject for Mesh<V, I> {
    fn vertices(&self) -> VerticesSource {
        self.geometry.vertex_buffer().into_vertices_source()
    }

    fn indices(&self) -> IndicesSource {
        IndicesSource::from(self.geometry.index_buffer())
    }

    fn program(&self) -> &Program {
        &self.program
    }

    fn model_transform(&self) -> (Matrix4<f32>, Matrix3<f32>) {
        let transform = Similarity3::new_with_rotation_matrix(
            self.position.to_vector(),
            self.orientation,
            self.scale);
        let model = transform.to_homogeneous();
        let model_inv_trans_3 = FromHomogeneous::from(&model);

        (model, model_inv_trans_3)
    }
}
