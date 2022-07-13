use std::ffi::{CStr, CString};

use gl::types::{GLenum, GLuint};

use crate::{
    generation_vec::GenerationVec,
    renderer::{shader::ShaderStorage, GraphicsStorage, Shader},
    RendererError,
};

impl ShaderStorage for GenerationVec<Shader, GLShader> {
    fn new_vertex(&mut self, source: &str) -> Result<crate::Handle<Shader>, RendererError> {
        let shader = GLShader::new_vertex(source)?;

        Ok(self.push(shader))
    }

    fn new_fragment(&mut self, source: &str) -> Result<crate::Handle<Shader>, RendererError> {
        let shader = GLShader::new_fragment(source)?;

        Ok(self.push(shader))
    }
}

pub struct GLShader {
    pub(super) id: GLuint,
    pub(super) kind: GLenum,
}

impl Drop for GLShader {
    fn drop(&mut self) {
        if self.id > 0 {
            unsafe { gl::DeleteShader(self.id) };
        }
    }
}

impl GLShader {
    pub(super) fn new_vertex(source: &str) -> Result<Self, RendererError> {
        Self::with_kind(gl::VERTEX_SHADER, source)
    }

    pub(super) fn new_fragment(source: &str) -> Result<Self, RendererError> {
        Self::with_kind(gl::FRAGMENT_SHADER, source)
    }

    fn with_kind(kind: GLenum, source: &str) -> Result<Self, RendererError> {
        let gl_shader = Self {
            id: unsafe { gl::CreateShader(kind) },
            kind,
        };

        let mut compile_status = 0;
        let len = [source.as_bytes().len() as i32];
        let source = CString::new(source).unwrap();

        let shader_array = [source.as_ptr()];
        unsafe {
            gl::ShaderSource(
                gl_shader.id,
                1,
                shader_array.as_ptr(),
                len.as_ptr() as *const i32,
            );
            gl::CompileShader(gl_shader.id);
            gl::GetShaderiv(gl_shader.id, gl::COMPILE_STATUS, &mut compile_status);
        }

        if compile_status != 0 {
            Ok(gl_shader)
        } else {
            //need to do some annoying dance to get the actual compile error for the shader through
            //the ffi nonsense
            let mut error_length = 0;
            unsafe {
                gl::GetShaderiv(gl_shader.id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let mut error_string: Vec<u8> = Vec::with_capacity(error_length as usize + 1);

            //fill error string with empty spaces
            error_string.extend([b' '].iter().cycle().take(error_length as usize));

            unsafe {
                gl::GetShaderInfoLog(
                    gl_shader.id,
                    error_length,
                    std::ptr::null_mut(),
                    error_string.as_ptr() as *mut gl::types::GLchar,
                );
            }
            let reason = String::from_utf8_lossy(&error_string).to_string();

            Err(RendererError::FailedToCompileShader { error: reason })
        }
    }
}
