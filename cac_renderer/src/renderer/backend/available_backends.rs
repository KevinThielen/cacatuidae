
enum AvailableBackends {
    #[cfg(feature = "opengl")]
    OpenGL(opengl::OpenGLRenderer),
    #[cfg(feature = "headless")]
    Headless(headless::HeadlessRenderer),
}

impl AvailableBackends {
    pub fn new(
        window_handle: &impl raw_window_handle::HasRawWindowHandle,
    ) -> Result<Self, RendererError> {
        let backend = Err(RendererError::NoAvailableBackend);

        cfg_if::cfg_if! {
        if #[cfg(all(feature="headless", test))]
        {
            //get rid of unused warning because headless doesn't need a window_handle
            let _ = window_handle;
            // There is nos reason to create a proper window or graphics context in the test cases
            // that are meant to run in a CI environment
            if let Ok(backend) = headless::HeadlessRenderer::new() {
                 return Ok(renderer);
            }
        }
        else if #[cfg(unix)] {
            // Unix order is Vulkan > OpenGL latest ver > OpenGL 33 > Headless

            #[cfg(feature="vulkan")] {
            }
            #[cfg(feature="opengl")] {
                match opengl::OpenGLRenderer::new(window_handle, (4, 5)) {
                  Ok(renderer) => return Ok(Self::OpenGL(renderer)),
                  Err(e) => log::warn!("Couldn't create preferred Context [OpenGL 4.5 Core]: {e}"),
                }
                match opengl::OpenGLRenderer::new(window_handle, (3, 3)) {
                  Ok(renderer) => return Ok(Self::OpenGL(renderer)),
                  Err(e) => log::warn!("Couldn't create Fallback Context [OpenGL 3.3 Core]: {e}"),
                }
            }
            #[cfg(feature="headless")] {

            }
        }
        else {
        todo!()
        }
        }

        backend
    }
}
