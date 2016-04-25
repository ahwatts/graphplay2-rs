use glium::Program;
use glium::backend::Facade;
use glium::program::BlockLayout;
use glium::uniforms::{UniformBlock, LayoutMismatchError};

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
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub specular_exp: f32,
}

impl Default for LightProperties {
    fn default() -> LightProperties {
        LightProperties {
            enabled:      false,
            position:     [ 0.0, 0.0, 0.0 ],
            color:        [ 0.0, 0.0, 0.0, 1.0],
            specular_exp: 0.0,
        }
    }
}

// implement_uniform_block!(LightProperties, enabled, position, color, specular_exp);

impl UniformBlock for LightProperties {
    fn matches(layout: &BlockLayout, base_offset: usize) -> Result<(), LayoutMismatchError> {
        if let &BlockLayout::Struct { ref members } = layout {
            for &(ref name, ref member_layout) in members.iter() {
                match name.as_ref() {
                    "enabled" => try! {
                        <bool as UniformBlock>::matches(member_layout, base_offset)
                            .map_err(|e| LayoutMismatchError::MemberMismatch {
                                member: "enabled".to_owned(),
                                err: Box::new(e),
                            })
                    },
                    "position" => try! {
                        <[f32; 3] as UniformBlock>::matches(member_layout, base_offset + 16)
                            .map_err(|e| LayoutMismatchError::MemberMismatch {
                                member: "position".to_owned(),
                                err: Box::new(e),
                            })
                    },
                    "color" => try! {
                        <[f32; 4] as UniformBlock>::matches(member_layout, base_offset + 32)
                            .map_err(|e| LayoutMismatchError::MemberMismatch {
                                member: "color".to_owned(),
                                err: Box::new(e),
                            })
                    },
                    "specular_exp" => try! {
                        <f32 as UniformBlock>::matches(member_layout, base_offset + 48)
                            .map_err(|e| LayoutMismatchError::MemberMismatch {
                                member: "specular_exp".to_owned(),
                                err: Box::new(e),
                            })
                    },
                    _ => {
                        return Err(LayoutMismatchError::MissingField {
                            name: name.clone(),
                        })
                    }
                }
            }
            Ok(())
        } else {
            Err(LayoutMismatchError::LayoutMismatch {
                expected: layout.clone(),
                obtained: LightProperties::build_layout(base_offset),
            })
        }
    }

    fn build_layout(base_offset: usize) -> BlockLayout {
        BlockLayout::Struct {
            members: vec! [
                ("enabled".to_owned(),      <bool     as UniformBlock>::build_layout(base_offset)),
                ("position".to_owned(),     <[f32; 3] as UniformBlock>::build_layout(base_offset + 16)),
                ("color".to_owned(),        <[f32; 4] as UniformBlock>::build_layout(base_offset + 32)),
                ("specular_exp".to_owned(), <f32      as UniformBlock>::build_layout(base_offset + 48)),
            ],
        }
    }
}

pub const MAX_LIGHTS: usize = 10;

#[derive(Clone, Copy, Debug, Default)]
pub struct LightListBlock {
    pub lights: [LightProperties; MAX_LIGHTS]
}

implement_uniform_block!(LightListBlock, lights);
