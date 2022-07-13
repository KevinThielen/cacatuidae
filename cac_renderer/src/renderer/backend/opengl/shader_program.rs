use gl::types::GLuint;

use crate::{
    generation_vec::GenerationVec,
    renderer::{shader::ProgramStorage, ShaderProgram},
    RendererError,
};

use super::GLShader;

impl ProgramStorage for GenerationVec<ShaderProgram, GLShaderProgram> {
    type VertexShader = GLShader;
    type FragmentShader = GLShader;

    fn new_program(
        &mut self,
        vertex_shader: &Self::VertexShader,
        fragment_shader: &Self::FragmentShader,
    ) -> Result<crate::Handle<ShaderProgram>, RendererError> {
        let program = GLShaderProgram::new(vertex_shader, fragment_shader)?;
        Ok(self.push(program))
    }
}

pub struct GLShaderProgram {
    id: GLuint,
}

impl Drop for GLShaderProgram {
    fn drop(&mut self) {
        if self.id > 0 {
            unsafe { gl::DeleteProgram(self.id) }
        }
    }
}

impl GLShaderProgram {
    pub(super) fn new(
        vertex_shader: &GLShader,
        fragment_shader: &GLShader,
    ) -> Result<Self, RendererError> {
        if vertex_shader.kind != gl::VERTEX_SHADER {
            return Err(RendererError::FailedToLinkProgram {
                error: "Argument vertex_shader is not a VertexShader".to_string(),
            });
        }
        if fragment_shader.kind != gl::FRAGMENT_SHADER {
            return Err(RendererError::FailedToLinkProgram {
                error: "Argument fragment_shader is not a FragmentShader".to_string(),
            });
        }

        let program = Self {
            id: unsafe { gl::CreateProgram() },
        };

        let mut link_status = 0;
        unsafe {
            gl::AttachShader(program.id, vertex_shader.id);
            gl::AttachShader(program.id, fragment_shader.id);
            gl::LinkProgram(program.id);
            gl::DetachShader(program.id, vertex_shader.id);
            gl::DetachShader(program.id, fragment_shader.id);

            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut link_status);
        }

        //link_status == 0 means there is a link error
        if link_status != 0 {
            unsafe { gl::UseProgram(program.id) };
            Ok(program)
        } else {
            let mut error_length = 0;

            unsafe {
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let mut error_string: Vec<u8> = Vec::with_capacity(error_length as usize + 1);
            error_string.extend([b' '].iter().cycle().take(error_length as usize));

            unsafe {
                gl::GetProgramInfoLog(
                    program.id,
                    error_length,
                    std::ptr::null_mut(),
                    error_string.as_mut_ptr() as *mut gl::types::GLchar,
                );
            }

            let reason = String::from_utf8_lossy(&error_string).to_string();
            Err(RendererError::FailedToLinkProgram { error: reason })
        }
    }
}
