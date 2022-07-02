use crate::backend::RendererBackend;

/// Renderer abstraction
///
/// Provides a high level API for the graphics backends. The backend can be chosen dynamically,
/// depending on the supported features, version and graphics device capabilities. See
/// [StaticRenderer] if you want to use a compile-time dependent one to avoid the boxing.
///
/// [Shader]s, [Mesh]es, [Texture]s can be created on the graphics device, while the renderer returns handles to them.
/// The user is responsible to releasing them with the handles. It is recommended to
/// keep track of the Handles with a human readable constant to associate them with a name or id.
/// The Renderer sorts and batches draw calls.

pub struct Renderer {
    pub(crate) backend: Box<dyn RendererBackend>,
}

impl Renderer {
    /// Creates a Renderer using a default backend. If the backend is not available or fails, it
    /// will try the next one
    ///
    /// # Default Backends
    /// | Platform | Default Backend | Status  |
    /// |----------|-----------------|---------|
    /// | Linux    |   OpenGL33      |  [WIP]  |
    /// | Wasm     |   WebGL2        |  [WIP]  |
    /// | Android  |   OpenGLes2     |  [WIP]  |
    /// | cfg(test)|   Headless      |  [WIP]  |
    /// | Fallback |   Wpgu          |  [WIP]  |
    ///
    ///
    /// # Errors
    /// InvalidWindowHandle

    pub fn new(
        window_handle: &impl raw_window_handle::HasRawWindowHandle,
    ) -> Result<Renderer, String> {
        let renderer = Err("No matching default backend available".to_string());

        cfg_if::cfg_if! {
        if #[cfg(all(feature="headless", test))]
        {
            // There is nos reason to create a proper window or graphics context in the test cases
            // that are meant to run in a CI environment
            if let Ok(renderer) = Self::new_headless() {
                 return Ok(renderer);
            }
        }
        else if #[cfg(unix)] {
            // Unix order is Vulkan > OpenGL latest ver > OpenGL 33 > Headless

            #[cfg(feature="vulkan")] {
            }
            #[cfg(feature="opengl")] {
                if let Ok(renderer) = Self::new_opengl() {
                    return Ok(renderer);
                }
            }
            #[cfg(feature="headless")] {
                if let Ok(renderer) = Self::new_headless() {
                    return Ok(renderer);
                }
            }
        }
        else {
        todo!()
        }
        }

        renderer
    }

    /// Prints a description of the context, including the version of the graphics library, driver
    /// information, etc.
    pub fn context_description(&self) -> String {
        self.backend.context_description()
    }

    /// Sets the clear color that is used by Self::clear()
    pub fn clear_color(&self, color: (f32, f32, f32, f32)) {}

    /// Sends the sorted and batched draw commands to the backend.
    /// Potentially swaps the back and front buffer.
    pub fn update(&mut self) {}

    /// Create a Mesh out of MeshBuffers
    pub fn create_mesh(&mut self, mesh_buffers: &[u8]) -> Result<&dyn crate::Mesh, String> {
        self.backend.create_mesh().ok_or_else(|| "Fuck".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Placeholder
    struct HeadlessWindow {}
    unsafe impl raw_window_handle::HasRawWindowHandle for HeadlessWindow {
        fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
            panic!("Trying to get the raw window handle of an Headlesswindow")
        }
    }

    #[test]
    fn create_mesh() {
        let mut renderer = Renderer::new(&HeadlessWindow {}).unwrap();
        let mesh = renderer.create_mesh(&[]);

        assert!(mesh.is_ok())
    }
}
