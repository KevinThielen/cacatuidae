use crate::{Handle, MaterialProperty};

use super::{
    buffer::CreateBuffer,
    shader::{CreateShader, CreateShaderProgram},
    vertex_layout::CreateVertexLayout,
    Material, Mesh, RenderTarget, Uniform,
};

pub mod headless;
pub mod opengl;

pub trait Context {
    type Context;
    type Buffer: CreateBuffer;
    type VertexLayout: CreateVertexLayout<Buffer = Self::Buffer>;
    type Shader: CreateShader;
    type ShaderProgram: CreateShaderProgram<VertexShader = Self::Shader, FragmentShader = Self::Shader>
        + Uniform;
}

/// Renderer Backend that is used by the [Renderer][crate::Renderer]
///
/// Every graphics API needs to implement this trait.
/// It is possible to use specific subsets or versions of a graphics API with different backends.
pub trait Backend {
    /// Returns a [String] that describes the backend and its graphics context.
    /// It could be the version of the graphics API used, certain extensions, features
    /// or driver details.
    fn context_description(&self) -> String;

    fn screen_target(&mut self) -> &mut dyn RenderTarget;

    fn draw(
        &mut self,
        mesh: Mesh,
        material: Handle<Material>,
        instance_properties: &[MaterialProperty],
    );

    fn update(&mut self);
}
