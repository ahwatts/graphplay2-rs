use std::iter;

use glium::{
    backend::Facade,
    implement_vertex,
    index::{Index, PrimitiveType},
    IndexBuffer, Vertex, VertexBuffer,
};
use nalgebra::Vector3;
use num::Bounded;

use crate::ply::Document;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCNVertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

impl PCNVertex {
    pub fn position_vec(&self) -> Vector3<f32> {
        Vector3::from_row_slice(&self.position)
    }

    pub fn normal_vec(&self) -> Vector3<f32> {
        Vector3::from_row_slice(&self.normal)
    }
}

implement_vertex!(PCNVertex, position, color, normal);

pub struct Geometry<V: Vertex, I: Index> {
    // vertices: Vec<V>,
    // indices: Vec<I>,
    vertex_buffer: VertexBuffer<V>,
    index_buffer: IndexBuffer<I>,
}

impl<V: Vertex, I: Index> Geometry<V, I> {
    pub fn new<F, IV, II>(
        facade: &F,
        draw_type: PrimitiveType,
        into_vertices: IV,
        into_indices: II,
    ) -> Geometry<V, I>
    where
        F: Facade,
        IV: Into<Vec<V>>,
        II: Into<Vec<I>>,
    {
        let vertices = into_vertices.into();
        let indices = into_indices.into();

        Geometry {
            vertex_buffer: VertexBuffer::new(facade, &vertices).unwrap(),
            index_buffer: IndexBuffer::new(facade, draw_type, &indices).unwrap(),
            // vertices: vertices,
            // indices: indices,
        }
    }

    pub fn vertex_buffer(&self) -> &VertexBuffer<V> {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &IndexBuffer<I> {
        &self.index_buffer
    }
}

fn find_or_add_vert<T: Vertex + PartialEq>(verts: &mut Vec<T>, new_vert: T) -> usize {
    match verts.iter().position(|v| *v == new_vert) {
        Some(index) => index,
        None => {
            let rv = verts.len();
            verts.push(new_vert);
            rv
        }
    }
}

static OCTOHEDRON_VERTICES: [PCNVertex; 6] = [
    PCNVertex { position: [  1.0,  0.0,  0.0, ], color: [ 1.0, 0.0, 0.0, 1.0 ], normal: [  1.0,  0.0,  0.0 ] },
    PCNVertex { position: [ -1.0,  0.0,  0.0, ], color: [ 1.0, 0.0, 0.0, 1.0 ], normal: [ -1.0,  0.0,  0.0 ] },
    PCNVertex { position: [  0.0,  0.0,  1.0, ], color: [ 0.0, 0.0, 1.0, 1.0 ], normal: [  0.0,  0.0,  1.0 ] },
    PCNVertex { position: [  0.0,  0.0, -1.0, ], color: [ 0.0, 0.0, 1.0, 1.0 ], normal: [  0.0,  0.0, -1.0 ] },
    PCNVertex { position: [  0.0, -1.0,  0.0, ], color: [ 0.0, 1.0, 0.0, 1.0 ], normal: [  0.0, -1.0,  0.0 ] },
    PCNVertex { position: [  0.0,  1.0,  0.0, ], color: [ 0.0, 1.0, 0.0, 1.0 ], normal: [  0.0,  1.0,  0.0 ] },
];

static OCTOHEDRON_ELEMENTS: [usize; 24] = [
    4, 0, 2, 4, 3, 0, 4, 1, 3, 4, 2, 1,
    5, 2, 0, 5, 0, 3, 5, 3, 1, 5, 1, 2,
];

pub fn octohedron<F: Facade>(facade: &F) -> Geometry<PCNVertex, u16> {
    let mut elems: Vec<u16> = vec!();
    let mut verts: Vec<PCNVertex> = vec!();

    let faces = OCTOHEDRON_ELEMENTS.chunks(3)
        .map(|e| [
            &OCTOHEDRON_VERTICES[e[0]],
            &OCTOHEDRON_VERTICES[e[1]],
            &OCTOHEDRON_VERTICES[e[2]],
        ]);

    // Replace the vertex normal, which doesn't make much sense for so
    // few vertices, with face normals. Ironically (in a Morrissettian
    // sense), this will "repeat" each vertex four times (once for
    // each face).
    for face in faces {
        let (v1, v2, v3) = (face[0], face[1], face[2]);
        let e1 = v2.position_vec() - v1.position_vec();
        let e2 = v3.position_vec() - v1.position_vec();
        let normal = e1.cross(&e2).normalize();

        let new_v1 = PCNVertex { position: v1.position, color: v1.color, normal: *normal.as_ref() };
        let new_v2 = PCNVertex { position: v2.position, color: v2.color, normal: *normal.as_ref() };
        let new_v3 = PCNVertex { position: v3.position, color: v3.color, normal: *normal.as_ref() };

        elems.push(find_or_add_vert(&mut verts, new_v1) as u16);
        elems.push(find_or_add_vert(&mut verts, new_v2) as u16);
        elems.push(find_or_add_vert(&mut verts, new_v3) as u16);
    }

    Geometry::new(facade, PrimitiveType::TrianglesList, verts, elems)
}

static WIREFRAME_CUBE_VERTICES: [PCNVertex; 8] = [
    PCNVertex { position: [  1.0,   1.0,   1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [  0.577350269,   0.577350269,   0.577350269 ] }, // 0
    PCNVertex { position: [  1.0,   1.0,  -1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [  0.577350269,   0.577350269,  -0.577350269 ] }, // 1
    PCNVertex { position: [  1.0,  -1.0,   1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [  0.577350269,  -0.577350269,   0.577350269 ] }, // 2
    PCNVertex { position: [  1.0,  -1.0,  -1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [  0.577350269,  -0.577350269,  -0.577350269 ] }, // 3
    PCNVertex { position: [ -1.0,   1.0,   1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [ -0.577350269,   0.577350269,   0.577350269 ] }, // 4
    PCNVertex { position: [ -1.0,   1.0,  -1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [ -0.577350269,   0.577350269,  -0.577350269 ] }, // 5
    PCNVertex { position: [ -1.0,  -1.0,   1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [ -0.577350269,  -0.577350269,   0.577350269 ] }, // 6
    PCNVertex { position: [ -1.0,  -1.0,  -1.0 ], color: [ 1.0, 1.0, 1.0, 1.0 ], normal: [ -0.577350269,  -0.577350269,  -0.577350269 ] }, // 7
];

static WIREFRAME_CUBE_ELEMENTS: [usize; 24] = [
    0, 2, 2, 6, 6, 4, 4, 0,
    0, 1, 2, 3, 4, 5, 6, 7,
    1, 3, 3, 7, 7, 5, 5, 1,
];

pub fn wireframe_cube<F: Facade>(facade: &F) -> Geometry<PCNVertex, u32> {
    let verts: Vec<PCNVertex> = WIREFRAME_CUBE_VERTICES.to_vec();
    let elems: Vec<u32> = WIREFRAME_CUBE_ELEMENTS.iter().map(|e| *e as u32).collect();
    Geometry::new(facade, PrimitiveType::LinesList, verts, elems)
}

pub fn load_ply<F: Facade>(facade: &F, filename: &str) -> Geometry<PCNVertex, u32> {
    let doc = Document::from_file(filename).unwrap();

    // Copy the vertex attributes.
    let vert_elem = doc
        .elements()
        .iter()
        .find(|e| e.name() == "vertex")
        .unwrap();
    let mut verts: Vec<PCNVertex> = iter::repeat(Default::default())
        .take(vert_elem.count() as usize)
        .collect();
    for p in vert_elem.properties() {
        match p.name() {
            "x" => for (v, p) in verts.iter_mut().zip(p.data().float_scalar().unwrap().iter()) { v.position[0] = *p as f32 },
            "y" => for (v, p) in verts.iter_mut().zip(p.data().float_scalar().unwrap().iter()) { v.position[1] = *p as f32 },
            "z" => for (v, p) in verts.iter_mut().zip(p.data().float_scalar().unwrap().iter()) { v.position[2] = *p as f32 },
            "nx" => for (v, p) in verts.iter_mut().zip(p.data().float_scalar().unwrap().iter()) { v.normal[0] = *p as f32 },
            "ny" => for (v, p) in verts.iter_mut().zip(p.data().float_scalar().unwrap().iter()) { v.normal[1] = *p as f32 },
            "nz" => for (v, p) in verts.iter_mut().zip(p.data().float_scalar().unwrap().iter()) { v.normal[2] = *p as f32 },
            "red"   => for (v, p) in verts.iter_mut().zip(p.data().int_scalar().unwrap().iter()) { v.color[0] = *p as f32 },
            "green" => for (v, p) in verts.iter_mut().zip(p.data().int_scalar().unwrap().iter()) { v.color[1] = *p as f32 },
            "blue"  => for (v, p) in verts.iter_mut().zip(p.data().int_scalar().unwrap().iter()) { v.color[2] = *p as f32 },
            "alpha" => for (v, p) in verts.iter_mut().zip(p.data().int_scalar().unwrap().iter()) { v.color[3] = *p as f32 },
            _ => {},
        }
    }

    // Copy the vertex elements.
    let faces = doc.elements().iter().find(|e| e.name() == "face").unwrap();
    let mut elems = vec![];
    for i in 0..(faces.count() as usize) {
        let p = faces
            .properties()
            .iter()
            .find(|p| p.name() == "vertex_indices")
            .unwrap();
        elems.extend(p.data().int_list().unwrap()[i].iter().map(|j| *j as u32));
    }

    // Postprocessing. Calculate the bounding box.
    let mut bb_min = Vector3::max_value();
    let mut bb_max = Vector3::min_value();
    for v in verts.iter() {
        bb_min = bb_min.inf(&v.position_vec());
        bb_max = bb_max.sup(&v.position_vec());
    }

    // Scale everything so that it's in the range -1 to 1 and centered.
    let bcenter = (bb_max + bb_min) / 2.0;
    let new_bb_max = bb_max - bcenter;
    let max_dim = new_bb_max.iter().fold(f32::min_value(), |m, &v| m.max(v));
    for v in verts.iter_mut() {
        v.position = *((v.position_vec() - bcenter) / max_dim).as_ref();
    }

    // Set the colors, assuming all vertices are opaque.
    for v in verts.iter_mut() {
        v.color[0] = (v.color[0] / v.color[3]) * v.position[0].abs();
        v.color[1] = (v.color[1] / v.color[3]) * v.position[1].abs();
        v.color[2] = (v.color[2] / v.color[3]) * v.position[2].abs();
        v.color[3] = 1.0;
    }

    Geometry::new(facade, PrimitiveType::TrianglesList, verts, elems)
}
