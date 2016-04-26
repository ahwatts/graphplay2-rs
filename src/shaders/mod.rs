use glium::Program;
use glium::backend::Facade;

pub fn unlit<F: Facade>(facade: &F) -> Program {
    Program::from_source(facade, UNLIT.vertex, UNLIT.fragment, None).unwrap()
}

pub fn lit<F: Facade>(facade: &F) -> Program {
    Program::from_source(facade, LIT.vertex, LIT.fragment, None).unwrap()
}

struct ShaderSource {
    pub vertex: &'static str,
    pub fragment: &'static str,
}

static UNLIT: ShaderSource = ShaderSource {
    vertex: include_str!("unlit_vertex.glsl"),
    fragment: include_str!("unlit_fragment.glsl"),
};

static LIT: ShaderSource = ShaderSource {
    vertex: include_str!("lit_vertex.glsl"),
    fragment: include_str!("lit_fragment.glsl"),
};

#[derive(Clone, Copy, Debug)]
pub struct ViewAndProjectionBlock {
    pub view:       [[f32; 4]; 4],
    pub view_inv:   [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

implement_uniform_block!(ViewAndProjectionBlock, view, view_inv, projection);

#[derive(Clone, Copy, Debug)]
pub struct LightProperties {
    pub enabled: bool,
    #[allow(dead_code)] padding1: [u32; 3],
    pub position: [f32; 3],
    #[allow(dead_code)] padding2: u32,
    pub color: [f32; 4],
    pub specular_exp: f32,
    #[allow(dead_code)] padding3: [u32; 3],
}

impl LightProperties {
    pub fn new(enabled: bool, position: [f32; 3], color: [f32; 4], specular_exp: f32) -> LightProperties {
        LightProperties {
            enabled: enabled,
            position: position,
            color: color,
            specular_exp: specular_exp,
            ..Default::default()
        }
    }
}

impl Default for LightProperties {
    fn default() -> LightProperties {
        LightProperties {
            enabled:      false,
            position:     [ 0.0, 0.0, 0.0 ],
            color:        [ 0.0, 0.0, 0.0, 1.0],
            specular_exp: 0.0,

            padding2:     Default::default(),
            padding1:     Default::default(),
            padding3:     Default::default(),
        }
    }
}

implement_uniform_block!(LightProperties, position, specular_exp, color, enabled);

pub const MAX_LIGHTS: usize = 10;

#[derive(Clone, Copy, Debug, Default)]
pub struct LightListBlock {
    pub lights: [LightProperties; MAX_LIGHTS]
}

implement_uniform_block!(LightListBlock, lights);
