use crate::{Handle, Renderer, RendererError};

use super::Context;

pub trait Uniform {
    fn get_uniform_location(&self, name: &str) -> u32;
    fn data_size(&self) -> usize;

    fn set_uniform_f32(&mut self, location: u32, value: &[f32]);
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Shader {}

pub trait CreateShader: Sized {
    fn with_vertex(source: &str) -> Result<Self, RendererError>;
    fn with_fragment(source: &str) -> Result<Self, RendererError>;
}

impl Shader {
    pub fn with_vertex<C: Context>(
        ctx: &mut Renderer<C>,
        source: &str,
    ) -> Result<Handle<Self>, RendererError> {
        let shader = C::Shader::with_vertex(source)?;
        Ok(ctx.shaders.push(shader))
    }
    pub fn with_fragment<C: Context>(
        ctx: &mut Renderer<C>,
        source: &str,
    ) -> Result<Handle<Self>, RendererError> {
        let shader = C::Shader::with_fragment(source)?;
        Ok(ctx.shaders.push(shader))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ShaderProgram {}

impl ShaderProgram {
    pub fn new<C: Context>(
        ctx: &mut Renderer<C>,
        vertex_shader: Handle<Shader>,
        fragment_shader: Handle<Shader>,
    ) -> Result<Handle<Self>, RendererError> {
        let vertex_shader =
            ctx.shaders
                .get(vertex_shader)
                .ok_or(RendererError::ResourceNotFound {
                    resource: "vertex shader".to_string(),
                })?;
        let fragment_shader =
            ctx.shaders
                .get(fragment_shader)
                .ok_or(RendererError::ResourceNotFound {
                    resource: "fragment shader".to_string(),
                })?;

        let program = C::ShaderProgram::new(vertex_shader, fragment_shader)?;

        Ok(ctx.programs.push(program))
    }

    pub fn from_sources<C: Context>(
        ctx: &mut Renderer<C>,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<Handle<Self>, RendererError> {
        let vertex_shader = Shader::with_vertex(ctx, vertex_shader)?;
        let fragment_shader = Shader::with_fragment(ctx, fragment_shader)?;

        let program = Self::new(ctx, vertex_shader, fragment_shader)?;

        ctx.shaders.remove(vertex_shader);
        ctx.shaders.remove(fragment_shader);

        Ok(program)
    }
}

pub trait CreateShaderProgram: Sized {
    type VertexShader;
    type FragmentShader;

    fn new(
        vertex_shader: &Self::VertexShader,
        fragment_shader: &Self::FragmentShader,
    ) -> Result<Self, RendererError>;
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
