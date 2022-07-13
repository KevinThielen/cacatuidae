use crate::AttributeSemantic;

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
    ConversionError {
        error: String,
    },
    ResourceNotFound {
        resource: String,
    },
    AttributeLocationOverMax {
        location: u8,
        max: u8,
        semantic: AttributeSemantic,
    },
    AttributeHasNoLocation {
        semantic: AttributeSemantic,
    },
    FailedToCompileShader {
        error: String,
    },
    FailedToLinkProgram {
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
            RendererError::ConversionError { error } => write!(f, "Conversion Failed: {error}"),
            RendererError::ResourceNotFound { resource } => {
                write!(f, "Couldn't find resource: {resource}")
            }
            RendererError::AttributeLocationOverMax {
                location,
                max,
                semantic,
            } => write!(f, "{semantic} >= {max}"),
            RendererError::AttributeHasNoLocation { semantic } => write!(f, "{semantic}"),
            RendererError::FailedToCompileShader { error } => {
                write!(f, "Failed to compile shader: {error}")
            }
            RendererError::FailedToLinkProgram { error } => {
                write!(f, "Failed to link shaderprogram: {error}")
            }
        }
    }
}
