use gl::types::{GLenum, GLint, GLuint};

use crate::{
    generation_vec::GenerationVec, renderer::LayoutStorage, RendererError, VertexAttribute,
    VertexLayout,
};

use super::buffer::GLBuffer;

impl LayoutStorage for GenerationVec<VertexLayout, Vao> {
    type Buffer = GLBuffer;

    fn new(
        &mut self,
        buffer_attributes: &[crate::BufferAttributes<Self::Buffer>],
    ) -> Result<crate::Handle<VertexLayout>, RendererError> {
        let mut vao = Vao::new();

        for buffer_attr in buffer_attributes {
            vao.set_buffer_attributes(
                &buffer_attr.buffer,
                buffer_attr.attributes,
                buffer_attr.offset,
            )?;
        }

        Ok(self.push(vao))
    }
}

impl VertexAttribute {
    fn as_gl_enum(&self) -> GLenum {
        use crate::VertexAttributeKind::*;
        match self.semantic.kind() {
            F32 => gl::FLOAT,
            Vec2 => gl::FLOAT,
            Vec3 => gl::FLOAT,
            Vec4 => gl::FLOAT,
        }
    }
}

static mut MAX_ATTRIBUTES: Option<GLint> = None;

#[derive(Debug)]
pub struct Vao {
    id: GLuint,
    pub(super) has_indices: bool,
}

impl Vao {
    pub(crate) fn new() -> Self {
        Self {
            id: unsafe {
                let mut vao = 0;
                gl::GenVertexArrays(1, &mut vao as *mut GLuint);
                vao
            },
            has_indices: false,
        }
    }

    pub(super) fn set_buffer_attributes(
        &mut self,
        buffer: &GLBuffer,
        attributes: &[VertexAttribute],
        offset: usize,
    ) -> Result<(), RendererError> {
        let mut max_attributes = 0;
        unsafe {
            if MAX_ATTRIBUTES.is_none() {
                gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_attributes as *mut GLint);
                MAX_ATTRIBUTES = Some(max_attributes);
            } else {
                max_attributes = MAX_ATTRIBUTES.unwrap_or(0);
            }
        }

        if buffer.kind == gl::ELEMENT_ARRAY_BUFFER {
            self.has_indices = true;
        }

        self.bind();
        buffer.bind();

        for attr in attributes.iter() {
            if let Some(location) = attr.semantic.location() {
                if location as i32 >= max_attributes {
                    return Err(RendererError::AttributeLocationOverMax {
                        location,
                        semantic: attr.semantic,
                        max: max_attributes as u8,
                    });
                }
                let offset = offset + attr.offset;
                unsafe {
                    gl::EnableVertexAttribArray(location.into());
                    gl::VertexAttribPointer(
                        location.into(),
                        attr.semantic.kind().components().into(),
                        attr.as_gl_enum(),
                        if attr.normalized { gl::TRUE } else { gl::FALSE },
                        attr.stride as GLint,
                        offset as *const usize as *const std::ffi::c_void,
                    )
                }
            } else {
                return Err(RendererError::AttributeHasNoLocation {
                    semantic: attr.semantic,
                });
            }
        }

        Ok(())
    }

    pub(super) fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        if self.id > 0 {}
    }
}
