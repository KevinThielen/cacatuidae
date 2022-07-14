use std::ffi::CStr;

use gl::types::GLenum;
use render_target::ScreenTarget;

use crate::{
    generation_vec::GenerationVec,
    renderer::{vertex_layout::VertexLayout, Shader, ShaderProgram},
    Buffer, Mesh, Primitive, RenderTarget, Renderer, RendererError,
};

mod mesh;
mod render_target;

mod vertex_array;
use vertex_array::Vao;

mod buffer;
use buffer::GLBuffer;

mod shader;
use shader::GLShader;

mod shader_program;
use shader_program::GLShaderProgram;

use super::Context;

pub struct OpenGLContext {
    context: raw_gl_context::GlContext,
    screen_target: ScreenTarget,

    mesh_to_draw: Option<Mesh>,
}

impl Renderer<OpenGLContext> {
    pub fn new(
        window: &impl raw_window_handle::HasRawWindowHandle,
        version: (u8, u8),
    ) -> Result<Self, RendererError> {
        let context = OpenGLContext::new(window, version)?;

        Ok(Self {
            context,
            buffers: GenerationVec::with_capacity(10),
            layouts: GenerationVec::with_capacity(5),
            shaders: GenerationVec::with_capacity(10),
            programs: GenerationVec::with_capacity(5),
            materials: GenerationVec::with_capacity(10),
        })
    }
}

impl From<Primitive> for GLenum {
    fn from(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
            Primitive::Lines => gl::LINES,
            Primitive::LineStrip => gl::LINE_STRIP,
            Primitive::Points => gl::POINTS,
        }
    }
}

impl OpenGLContext {
    pub fn new(
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

        Ok(OpenGLContext {
            context,
            screen_target: ScreenTarget::default(),
            mesh_to_draw: None,
        })
    }
}

impl Context for OpenGLContext {
    type Context = Self;
    type BufferStorage = GenerationVec<Buffer, GLBuffer>;
    type LayoutStorage = GenerationVec<VertexLayout, Vao>;
    type ShaderStorage = GenerationVec<Shader, GLShader>;
    type ProgramStorage = GenerationVec<ShaderProgram, GLShaderProgram>;
}

impl crate::Renderer<OpenGLContext> {
    /// Creates a renderer using the OpenGL backend.
    /// By default, it will try to create a 3.3 or newer Core Context.
    /// It will also set the debug callbacks in debug builds
    pub fn new_opengl(
        window: &impl raw_window_handle::HasRawWindowHandle,
        version: (u8, u8),
    ) -> Result<Self, RendererError> {
        Self::new(window, version)
    }
}

impl super::Backend for Renderer<OpenGLContext> {
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
        &mut self.context.screen_target
    }

    fn draw(&mut self, mesh: crate::Mesh) {
        self.context.mesh_to_draw = Some(mesh);
    }

    fn update(&mut self) {
        self.context.screen_target.clear();
        if let Some(mesh_to_draw) = &self.context.mesh_to_draw {
            if let Some(vertex_array) = self.layouts.get_mut(mesh_to_draw.vertex_layout) {
                vertex_array.bind();

                if vertex_array.has_indices {
                    let start_index = mesh_to_draw.start_index as i32;
                    unsafe {
                        gl::DrawElements(
                            mesh_to_draw.primitive.into(),
                            mesh_to_draw.count as i32,
                            gl::UNSIGNED_BYTE,
                            start_index as *const i32 as *const std::ffi::c_void,
                        );
                    }
                } else {
                    unsafe {
                        gl::DrawArrays(
                            mesh_to_draw.primitive.into(),
                            mesh_to_draw.start_index as i32,
                            mesh_to_draw.count as i32,
                        );
                    }
                }
            }
        }
        self.context.context.swap_buffers();
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
