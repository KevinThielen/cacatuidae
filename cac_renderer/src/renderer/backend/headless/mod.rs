#![cfg(feature = "headless")]

use crate::Renderer;

//use self::mesh::Mesh;

//mod mesh;
mod render_target;
use render_target::RenderTarget;

/// Headless Backend
///
/// No graphics calls are send to the graphics device.
/// Instead, it logs them and creates dummy values for the exposed resources.
/// This allows the renderer to run tests and miri without creating expensive graphics contexts or
/// using sys calls.
pub struct Headless {
    //meshes: Vec<Mesh>,
    screen_target: RenderTarget,
}

impl Headless {
    pub(crate) fn new() -> Self {
        Self {
            //meshes: Vec::with_capacity(5),
            screen_target: RenderTarget::default(),
        }
    }
}

impl Renderer {
    /// New Headless Renderer
    /// So far there is no reason for it to ever fail, but the Result return type is consistent
    /// with the other renderers and avoid the "following code can't be reached" warning
    pub fn new_headless() -> Result<Self, String> {
        Ok(Self {
            backend: Box::new(Headless::new()),
        })
    }
}

impl super::RendererBackend for Headless {
    /// Returns a String with the content "Headless".       
    ///
    /// It's not very useful in this case, but returns driver, versions and extensions in the real
    /// backends.
    ///
    /// ```
    /// # use cac_renderer::{Renderer};
    /// # let renderer = Renderer::new_headless().unwrap();
    /// assert_eq!(renderer.context_description(), "Headless Renderer".to_string());
    /// ```
    fn context_description(&self) -> String {
        "Headless Renderer".to_string()
    }

    fn screen_target(&mut self) -> &mut dyn crate::RenderTarget {
        &mut self.screen_target
    }

    //fn create_mesh(&mut self) -> Option<&dyn crate::Mesh> {
    //self.meshes.push(Mesh::new());
    //if let Some(v) = self.meshes.last() {
    //Some(v)
    //} else {
    //None
    //}
    //}
}
