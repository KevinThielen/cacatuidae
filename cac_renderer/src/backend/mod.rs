mod headless;

/// Provided backends
///
/// Alternatively, implement [RendererBackend] to create a custom backend.
pub mod backends {
    pub use super::headless::Headless;
}

/// Renderer Backend that is used by the [Renderer][crate::Renderer]
///
/// Every graphics API needs to implement this trait.
/// It is possible to use specific subsets or versions of a graphics API with different backends.
pub trait RendererBackend {
    /// Returns a [String] that describes the backend and its graphics context.
    /// It could be the version of the graphics API used, certain extensions, features
    /// or driver details.
    fn context_description(&self) -> String;
}
