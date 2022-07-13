use super::{BufferStorage, LayoutStorage, Mesh, ProgramStorage, RenderTarget, ShaderStorage};

#[cfg(feature = "headless")]
mod headless;
#[cfg(feature = "opengl")]
pub mod opengl;

pub trait Context {
    type Context;
    type BufferStorage: BufferStorage;
    type LayoutStorage: LayoutStorage;
    type ShaderStorage: ShaderStorage;
    type ProgramStorage: ProgramStorage;
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

    fn draw(&mut self, mesh: Mesh);

    fn update(&mut self);
}
