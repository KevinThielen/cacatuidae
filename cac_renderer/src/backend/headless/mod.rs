/// Headless Backend
///
/// No graphics calls are send to the graphics device.
/// Instead, it logs them and creates dummy values for the exposed resources.
/// This allows the renderer to run tests and miri without creating expensive graphics contexts or
/// using sys calls.

pub struct Headless {}

impl Headless {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl super::RendererBackend for Headless {
    /// Returns a String with the content "Headless".       
    ///
    /// It's not very useful in this case, but returns driver, versions and extensions in the real
    /// backends.
    ///
    /// ```
    /// # use cac_renderer::{RendererBackend, backends::Headless};
    /// # let renderer = Headless{};
    /// assert_eq!(renderer.context_description(), "Headless Renderer".to_string());
    /// ```
    fn context_description(&self) -> String {
        "Headless Renderer".to_string()
    }
}
