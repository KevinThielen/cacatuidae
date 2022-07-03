use crate::Mesh;

#[cfg(feature = "headless")]
mod headless;
#[cfg(feature = "opengl")]
mod opengl;

/// Renderer Backend that is used by the [Renderer][crate::Renderer]
///
/// Every graphics API needs to implement this trait.
/// It is possible to use specific subsets or versions of a graphics API with different backends.
pub trait RendererBackend {
    /// Returns a [String] that describes the backend and its graphics context.
    /// It could be the version of the graphics API used, certain extensions, features
    /// or driver details.
    fn context_description(&self) -> String;

    /// Creates a mesh on the graphics device
    /// The Renderer will save it and keep track of a handle
    fn create_mesh(&mut self) -> Option<&dyn Mesh>;
}
