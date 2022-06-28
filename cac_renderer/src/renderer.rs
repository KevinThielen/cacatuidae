use crate::backend::RendererBackend;

/// Renderer abstraction
///
/// Provides a high level API for the graphics backends.
///
/// [Shader]s, [Mesh]es, [Texture]s can be created on the graphics device, while the renderer returns handles to them.
/// The user is responsible to releasing them with the handles. It is recommended to
/// keep track of the Handles with a human readable constant to associate them with a name or id.

pub struct Renderer {
    backend: Box<dyn RendererBackend>,
}

impl Renderer {
    /// Creates a Renderer using a default backend.
    ///
    /// # Default Backends
    /// | Platform | Default Backend | Status  |
    /// |----------|-----------------|---------|
    /// | Linux    |   OpenGL33      |  [WIP]  |
    /// | Wasm     |   WebGL2        |  [WIP]  |
    /// | Android  |   GLes2         |  [WIP]  |
    /// | cfg(test)|   Headless      |  [WIP]  |
    ///
    ///
    /// # Errors
    /// InvalidWindowHandle
    ///
    pub fn new(window_handle: &impl raw_window_handle::HasRawWindowHandle) -> Self {
        use crate::backends;
        let backend = Box::new(backends::Headless::new());

        Self { backend }
    }

    /// Sets the clear color that is used by Self::clear()
    pub fn clear_color(&self, color: (f32, f32, f32, f32)) {}

    /// Sends the sorted and batched draw commands to the backend.
    /// Potentially swaps the back and front buffer.
    pub fn update(&mut self) {}
}
