use crate::{Handle, RendererError};

#[derive(Copy, Clone, Debug)]
pub struct Shader {}

#[derive(Copy, Clone, Debug)]
pub struct ShaderProgram {}

pub trait ShaderStorage {
    //TODO: new with handle, new with name
    fn new_vertex(&mut self, source: &str) -> Result<Handle<Shader>, RendererError>;
    fn new_fragment(&mut self, source: &str) -> Result<Handle<Shader>, RendererError>;
}

pub trait ProgramStorage {
    type VertexShader;
    type FragmentShader;

    fn new_program(
        &mut self,
        vertex_shader: &Self::VertexShader,
        fragment_shader: &Self::FragmentShader,
    ) -> Result<Handle<ShaderProgram>, RendererError>;
}
