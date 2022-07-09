mod backend;

use backend::RendererBackend;

mod mesh;
pub use mesh::Mesh;

mod render_target;
pub use render_target::{ClearFlags, RenderTarget};

use crate::RendererError;

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
    //meshes: GenerationVec<MeshHandle, Mesh>,
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
    ) -> Result<Self, RendererError> {
        let renderer = Err(RendererError::NoAvailableBackend);

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
                match Self::new_opengl(window_handle, (4, 5)) {
                  Ok(renderer) => return Ok(renderer),
                  Err(e) => log::warn!("Couldn't create preferred Context [OpenGL 4.5 Core]: {e}"),
                }
                match Self::new_opengl(window_handle, (3, 3)) {
                  Ok(renderer) => return Ok(renderer),
                  Err(e) => log::warn!("Couldn't create Fallback Context [OpenGL 3.3 Core]: {e}"),
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

    //pub fn create_mesh<T>(&mut self, mesh_buffers: &[T]) -> Result<Handle<Mesh>, RendererError> {
    //self.backend.create_mesh(mesh_buffers)
    //}
}

#[cfg(test)]
mod test {
    use crate::Color32;

    use super::*;

    #[test]
    fn create_mesh() {
        let mut renderer = Renderer::new_headless().unwrap();
        const VERTICES: [f32; 9] = [0.0; 9];

        //let mesh_handle = renderer.create_mesh(&VERTICES).unwrap();
        //assert_eq!(renderer.meshes.len(), 1);
    }
}
