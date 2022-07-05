mod backend;
use backend::RendererBackend;

mod mesh;

mod render_target;
pub use render_target::{ClearFlags, RenderTarget};

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
    //draw_buckets: Vec<DrawBucket>,
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
            //get rid of unused warning because headless doesn't need a window_handle
            let _ = window_handle;
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
                if let Ok(renderer) = Self::new_opengl(window_handle) {
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

    /// Returns the screen as render target.
    pub fn screen_target(&mut self) -> &mut dyn RenderTarget {
        self.backend.screen_target()
    }

    /// Prints a description of the context, including the version of the graphics library, driver
    /// information, etc.
    pub fn context_description(&self) -> String {
        self.backend.context_description()
    }

    /// Sends the sorted and batched draw commands to the backend.
    /// Potentially swaps the back and front buffer.
    pub fn update(&mut self) {
        self.backend.update()
    }

    // Create a Mesh out of MeshBuffers
    //pub fn create_mesh(&mut self, mesh_buffers: &[u8]) -> Result<&dyn Mesh, String> {
    //self.backend.create_mesh().ok_or_else(|| "Fuck".to_string())
    //}
}

#[cfg(test)]
mod test {
    use crate::Color32;

    use super::*;

    #[test]
    fn clear_screen() {
        let mut renderer = Renderer::new_headless().unwrap();

        let target = renderer.screen_target();
        target.set_clear_color(Color32::from_rgb(0.2, 0.3, 0.8));
    }
}
