#![cfg(feature = "opengl")]
use std::ffi::CStr;

use gl::types::GLenum;
use render_target::ScreenTarget;

use crate::{
    generation_vec::GenerationVec,
    renderer::{vertex_layout::VertexLayout, Material, Uniform},
    Handle, MaterialProperty, Mesh, Primitive, RenderTarget, Renderer, RendererError,
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

    draw_list: Vec<DrawCommand>,
}

struct DrawCommand {
    mesh: Mesh,
    material: Handle<Material>,
    instance_data: Vec<(u32, Vec<f32>)>,
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
            draw_list: Vec::with_capacity(100),
        })
    }
}

impl Context for OpenGLContext {
    type Buffer = GLBuffer;
    type VertexLayout = Vao;
    type Context = Self;
    type Shader = GLShader;
    type ShaderProgram = GLShaderProgram;
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

    fn draw(
        &mut self,
        mesh: crate::Mesh,
        material: Handle<Material>,
        instance_properties: &[MaterialProperty],
    ) {
        let mut instance_data = Vec::with_capacity(instance_properties.len());
        for prop in instance_properties {
            let loc = match prop.property {
                crate::PropertyId::Name(name) => {
                    log::warn!("Using uniform names for instance properties is super slow. Use locations instead!");
                    if let Some(material) = self.materials.get(material) {
                        if let Some(program) = self.programs.get(material.program) {
                            program.get_uniform_location(name)
                        } else {
                            log::warn!("Program not found!");
                            0
                        }
                    } else {
                        log::warn!("Material not found!");
                        0
                    }
                }
                crate::PropertyId::Location(loc) => loc,
            };

            let mut data = Vec::with_capacity(12);

            match prop.value {
                crate::PropertyValue::F32(values) => {
                    values.iter().enumerate().for_each(|(index, v)| {
                        let bits = v.to_le_bytes();
                        let index = index * 4;
                        data.splice(index..(index + 4), bits);
                    });
                }
            }
            instance_data.push((loc, data));
        }

        self.context.draw_list.push(DrawCommand {
            mesh,
            material,
            instance_data: Vec::with_capacity(instance_properties.len()),
        });
    }

    fn update(&mut self) {
        self.context.screen_target.clear();

        let mut has_indices = false;
        let mut bound_vao = Handle::<VertexLayout>::new();
        let mut bound_material = Handle::<Material>::new();

        for command in &self.context.draw_list {
            if command.mesh.vertex_layout != bound_vao {
                if let Some(vertex_array) = self.layouts.get_mut(command.mesh.vertex_layout) {
                    vertex_array.bind();
                    has_indices = vertex_array.has_indices;
                    bound_vao = command.mesh.vertex_layout;
                } else {
                    log::warn!("Vertex Layout not found");
                    continue;
                }
            }

            if command.material != bound_material {
                if let Some(material) = self.materials.get(command.material) {
                    bound_material = command.material;
                    if let Some(program) = self.programs.get_mut(material.program) {
                        program.set_uniform_data(&material.data);

                        command.instance_data.iter().for_each(|(location, val)| {
                            program.set_uniform_f32(*location, val);
                        })
                    }
                }
            }

            if has_indices {
                let start_index = command.mesh.start_index as i32;
                unsafe {
                    gl::DrawElements(
                        command.mesh.primitive.into(),
                        command.mesh.count as i32,
                        gl::UNSIGNED_BYTE,
                        start_index as *const i32 as *const std::ffi::c_void,
                    );
                }
            } else {
                unsafe {
                    gl::DrawArrays(
                        command.mesh.primitive.into(),
                        command.mesh.start_index as i32,
                        command.mesh.count as i32,
                    );
                }
            }
        }

        self.context.context.swap_buffers();
        self.context.draw_list.clear();
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
