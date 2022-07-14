use std::mem::size_of;

use gl::types::{GLchar, GLuint};

use crate::{
    generation_vec::GenerationVec,
    renderer::{
        shader::{ProgramStorage, Uniform},
        ShaderProgram, UniformDescription, UniformKind,
    },
    Handle, RendererError,
};

use super::GLShader;

impl ProgramStorage for GenerationVec<ShaderProgram, GLShaderProgram> {
    type VertexShader = GLShader;
    type FragmentShader = GLShader;

    type ShaderProgram = GLShaderProgram;

    fn new_program(
        &mut self,
        vertex_shader: &Self::VertexShader,
        fragment_shader: &Self::FragmentShader,
    ) -> Result<crate::Handle<ShaderProgram>, RendererError> {
        let program = Self::ShaderProgram::new(vertex_shader, fragment_shader)?;
        Ok(self.push(program))
    }

    fn get(&self, program: Handle<ShaderProgram>) -> Option<&Self::ShaderProgram> {
        self.get(program)
    }

    fn get_mut(&mut self, program: Handle<ShaderProgram>) -> Option<&mut Self::ShaderProgram> {
        self.get_mut(program)
    }
}

pub struct GLShaderProgram {
    id: GLuint,
    data_size: usize,
    uniforms: Vec<UniformDescription>,
}

impl Uniform for GLShaderProgram {
    fn get_uniform_location(&self, name: &str) -> u32 {
        unsafe { gl::GetUniformLocation(self.id, name.as_ptr() as *const GLchar) as u32 }
    }

    fn data_size(&self) -> usize {
        self.data_size
    }

    fn uniforms(&self) -> &Vec<UniformDescription> {
        &self.uniforms
    }

    fn set_uniform_data(&mut self, data: &[u8]) {
        for uniform in &self.uniforms {
            let (location, count, value) = (
                uniform.location as i32,
                uniform.count as i32,
                &data[uniform.offset] as *const u8 as *const f32,
            );
            unsafe {
                match uniform.kind {
                    UniformKind::F32 => gl::Uniform1fv(location, count, value),
                    UniformKind::Mat4 => gl::UniformMatrix4fv(location, count, gl::FALSE, value),
                    UniformKind::Mat3 => gl::UniformMatrix3fv(location, count, gl::FALSE, value),
                    UniformKind::Mat2 => gl::UniformMatrix2fv(location, count, gl::FALSE, value),
                    UniformKind::Vec4 => gl::Uniform4fv(location, count, value),
                    UniformKind::Vec3 => gl::Uniform3fv(location, count, value),
                    UniformKind::Vec2 => gl::Uniform2fv(location, count, value),
                    UniformKind::Sampler2D => todo!(),
                }
            }
        }
    }
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

        let id = unsafe { gl::CreateProgram() };

        let mut link_status = 0;
        unsafe {
            gl::AttachShader(id, vertex_shader.id);
            gl::AttachShader(id, fragment_shader.id);
            gl::LinkProgram(id);
            gl::DetachShader(id, vertex_shader.id);
            gl::DetachShader(id, fragment_shader.id);

            gl::GetProgramiv(id, gl::LINK_STATUS, &mut link_status);
        }

        let (uniforms, data_size) = Self::get_uniforms(id);
        //link_status == 0 means there is a link error
        if link_status != 0 {
            unsafe { gl::UseProgram(id) };
            let program = Self {
                id,
                data_size,
                uniforms,
            };
            Ok(program)
        } else {
            let mut error_length = 0;

            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut error_length);
            }

            let mut error_string: Vec<u8> = Vec::with_capacity(error_length as usize + 1);
            error_string.extend([b' '].iter().cycle().take(error_length as usize));

            unsafe {
                gl::GetProgramInfoLog(
                    id,
                    error_length,
                    std::ptr::null_mut(),
                    error_string.as_mut_ptr() as *mut gl::types::GLchar,
                );
            }

            let reason = String::from_utf8_lossy(&error_string).to_string();
            Err(RendererError::FailedToLinkProgram { error: reason })
        }
    }

    fn get_uniforms(id: GLuint) -> (Vec<UniformDescription>, usize) {
        let mut uniform_count = 0;
        unsafe {
            gl::GetProgramiv(id, gl::ACTIVE_UNIFORMS, &mut uniform_count);
        }
        if uniform_count <= 0 {
            return (Vec::new(), 0);
        }

        let mut data_size = 0;
        let mut uniforms = Vec::with_capacity(uniform_count as usize);

        let mut texture_index = 0;
        const BUFFER_SIZE: usize = 256;

        //go through each uniform and read the data
        for index in 0..uniform_count {
            let mut uniform_count = 0;
            let mut name_length = 0;
            let mut uniform_kind = 0;
            let mut uniform_name = [b' '; BUFFER_SIZE];

            unsafe {
                gl::GetActiveUniform(
                    id,
                    index as u32,
                    BUFFER_SIZE as i32,
                    &mut name_length,
                    &mut uniform_count,
                    &mut uniform_kind,
                    uniform_name.as_mut_ptr() as *mut i8,
                )
            }

            //Actually gets the location. The index dunfortunately doesn't match the uniform
            //location. What a bummer.
            let location =
                unsafe { gl::GetUniformLocation(id, uniform_name.as_ptr() as *const i8) };

            //uniform is either not active(unused?) or not a "real" uniform
            if location < 0 {
                continue;
            }
            let uniform_len = uniform_count as usize;

            let uniform_kind = match uniform_kind {
                gl::FLOAT => UniformKind::F32,
                gl::FLOAT_VEC2 => UniformKind::Vec2,
                gl::FLOAT_VEC3 => UniformKind::Vec3,
                gl::FLOAT_VEC4 => UniformKind::Vec4,
                gl::FLOAT_MAT2 => UniformKind::Mat2,
                gl::FLOAT_MAT3 => UniformKind::Mat3,
                gl::FLOAT_MAT4 => UniformKind::Mat4,
                gl::SAMPLER_2D => UniformKind::Sampler2D,
                _ => todo!(),
            };

            //it's an array, so get rid of the [0] at the end
            //TODO: potentially driver dependent whether [0] is appended, so I need to look
            //into that
            if uniform_len > 1 {
                name_length -= 3;
            }

            let name = String::from_utf8_lossy(&uniform_name[0..name_length as usize]).to_string();

            //let texture_slot = match kind {
            //Kind::Sampler2D { len } => {
            //let slot = texture_index;
            //texture_index += len as u32;
            //Some(slot)
            //}
            //_ => None,
            //};

            let uniform_size = uniform_size_from_kind(uniform_kind, uniform_len);
            uniforms.push(UniformDescription {
                name,
                location: location as u32,
                kind: uniform_kind,
                count: uniform_len as u32,
                size: uniform_size,
                offset: data_size,
            });

            data_size += uniform_size;
        }

        (uniforms, data_size)
    }
}

fn uniform_size_from_kind(kind: UniformKind, count: usize) -> usize {
    let size = match kind {
        UniformKind::F32 => size_of::<gl::types::GLfloat>(),
        UniformKind::Sampler2D => todo!(),
        UniformKind::Mat2 => size_of::<gl::types::GLfloat>() * 4,
        UniformKind::Mat3 => size_of::<gl::types::GLfloat>() * 12,
        UniformKind::Mat4 => size_of::<gl::types::GLfloat>() * 16,
        UniformKind::Vec2 => size_of::<gl::types::GLfloat>() * 2,
        UniformKind::Vec3 => size_of::<gl::types::GLfloat>() * 3,
        UniformKind::Vec4 => size_of::<gl::types::GLfloat>() * 4,
    };

    size * count
}
