use glium::Program;
use glium::backend::Facade;

pub fn unlit<F: Facade>(facade: &F) -> Program {
    Program::from_source(facade, UNLIT.vertex, UNLIT.fragment, None).unwrap()
}

struct ShaderSource {
    pub vertex: &'static str,
    pub fragment: &'static str,
}

static UNLIT: ShaderSource = ShaderSource {
    vertex: include_str!("unlit_vertex.glsl"),
    fragment: include_str!("unlit_fragment.glsl"),
};

#[derive(Clone, Copy, Debug)]
pub struct ViewAndProjectionBlock {
    pub view:       [[f32; 4]; 4],
    pub view_inv:   [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

implement_uniform_block!(ViewAndProjectionBlock, view, view_inv, projection);
