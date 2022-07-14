use crate::{Handle, RendererError};

pub trait Uniform {
    fn get_uniform_location(&self, name: &str) -> u32;
    fn data_size(&self) -> usize;

    fn uniforms(&self) -> &Vec<UniformDescription>;
    fn set_uniform_data(&mut self, data: &[u8]);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum UniformKind {
    F32,
    Sampler2D,
    Mat4,
    Mat3,
    Mat2,
    Vec4,
    Vec3,
    Vec2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UniformDescription {
    pub name: String,
    pub location: u32,
    pub kind: UniformKind,
    pub count: u32,
    pub size: usize,
    pub offset: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Shader {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ShaderProgram {}

pub trait ShaderStorage {
    //TODO: new with handle, new with name
    fn new_vertex(&mut self, source: &str) -> Result<Handle<Shader>, RendererError>;
    fn new_fragment(&mut self, source: &str) -> Result<Handle<Shader>, RendererError>;
}

pub trait ProgramStorage {
    type VertexShader;
    type FragmentShader;
    type ShaderProgram: Uniform;

    fn new_program(
        &mut self,
        vertex_shader: &Self::VertexShader,
        fragment_shader: &Self::FragmentShader,
    ) -> Result<Handle<ShaderProgram>, RendererError>;

    fn get(&self, handle: Handle<ShaderProgram>) -> Option<&Self::ShaderProgram>;

    fn get_mut(&mut self, handle: Handle<ShaderProgram>) -> Option<&mut Self::ShaderProgram>;
}
