#[derive(Debug)]
/// Possible Errors returned by the Renderer.
pub enum RendererError {
    /// The selected target or features don't have any available backends, except the headless renderer.
    NoAvailableBackend,
    /// The backend was unable to create a valid context.
    FailedToCreateContext {
        /// Error Message returned by the backend.
        error: String,
    },
}

impl std::error::Error for RendererError {}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::NoAvailableBackend => {
                write!(f, "No backend available for target platform/features")
            }
            RendererError::FailedToCreateContext { error } => {
                write!(f, "Failed to Create Context: {error}")
            }
        }
    }
}
