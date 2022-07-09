use std::ffi::CStr;

use render_target::ScreenTarget;

use crate::{generation_vec::GenerationVec, Handle, RenderTarget, RendererError};

mod mesh;
mod render_target;

use mesh::Mesh;

struct OpenGLRenderer {
    context: raw_gl_context::GlContext,
    screen_target: ScreenTarget,
    meshes: GenerationVec<Handle<Mesh>>,
}

impl crate::Renderer {
    /// Creates a renderer using the OpenGL backend.
    /// By default, it will try to create a 3.3 or newer Core Context.
    /// It will also set the debug callbacks in debug builds
    pub fn new_opengl(
        window: &impl raw_window_handle::HasRawWindowHandle,
        version: (u8, u8),
    ) -> Result<Self, RendererError> {
        let context = raw_gl_context::GlContext::create(
            window,
            raw_gl_context::GlConfig {
                alpha_bits: 0,
                version,
                profile: raw_gl_context::Profile::Core,
                ..Default::default()
            },
        );

        let context = match context {
            Ok(context) => context,
            Err(gl_error) => {
                let message = match gl_error {
                    raw_gl_context::GlError::InvalidWindowHandle => "InvalidWindowHandle",
                    raw_gl_context::GlError::VersionNotSupported => "VersionNotSupported",
                    raw_gl_context::GlError::CreationFailed => "CreationFailed",
                };

                return Err(RendererError::FailedToCreateContext {
                    error: message.to_string(),
                });
            }
        };

        context.make_current();
        gl::load_with(|s| context.get_proc_address(s));

        //use debug callback for errors
        // it is supported on GL 4.3, so we need to check wether it or a fallback are loaded.
        if gl::DebugMessageCallback::is_loaded() {
            unsafe {
                gl::Enable(gl::DEBUG_OUTPUT);
                gl::DebugMessageCallback(Some(debug_callback), std::ptr::null());
            }
        } else {
            log::warn!("DebugMessageCallback is not loaded!")
        }

        let backend = OpenGLRenderer {
            context,
            screen_target: ScreenTarget::default(),
            meshes: GenerationVec::with_capacity(10),
        };

        Ok(Self {
            backend: Box::new(backend),
        })
    }
}

impl super::RendererBackend for OpenGLRenderer {
    fn context_description(&self) -> String {
        let vendor = unsafe { CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8) }
            .to_string_lossy()
            .to_owned();
        let renderer = unsafe { CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8) }
            .to_string_lossy()
            .to_owned();
        let version = unsafe { CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) }
            .to_string_lossy()
            .to_owned();
        let shading_ver =
            unsafe { CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8) }
                .to_string_lossy()
                .to_owned();

        format!("{vendor}\n{renderer}\n{version}\n{shading_ver}")
    }

    fn screen_target(&mut self) -> &mut dyn crate::RenderTarget {
        &mut self.screen_target
    }

    fn update(&mut self) {
        self.screen_target.clear();

        self.context.swap_buffers();
    }

    fn create_mesh(&mut self) -> Result<&mut dyn crate::Mesh, RendererError> {
        todo!()
    }
}

extern "system" fn debug_callback(
    source: u32,
    kind: u32,
    id: u32,
    severity: u32,
    _length: i32,
    message: *const i8,
    _user_param: *mut std::ffi::c_void,
) {
    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW SYSTEM",
        gl::DEBUG_SOURCE_OTHER => "OTHER",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD PARTY",
        _ => "UNKNOWN",
    };

    let kind = match kind {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED BEHAVIOUR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        _ => "UNKNOWN",
    };

    let error_message = unsafe { CStr::from_ptr(message).to_str().unwrap() };

    match severity {
        gl::DEBUG_SEVERITY_HIGH => log::error!("{id}: {kind} from {source}: {error_message}"),
        gl::DEBUG_SEVERITY_MEDIUM => log::warn!("{id}: {kind} from {source}: {error_message}"),
        gl::DEBUG_SEVERITY_LOW => log::info!("{id}: {kind} from {source}: {error_message}"),
        gl::DEBUG_SEVERITY_NOTIFICATION => {
            log::trace!("{id}: {kind} from {source}: {error_message}")
        }
        _ => log::trace!("{id}: {kind} from {source}: {error_message}"),
    };
}
