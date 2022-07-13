use gl::types::{GLenum, GLuint};

use crate::{
    generation_vec::GenerationVec, renderer::buffer::BufferStorage, Buffer, BufferUsage, Handle,
    RendererError,
};

#[derive(Debug)]
pub struct GLBuffer {
    pub(super) kind: GLuint,
    id: GLuint,
}

impl BufferStorage for GenerationVec<Buffer, GLBuffer> {
    fn new_vertex<T>(
        &mut self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Handle<Buffer>, RendererError> {
        let gl_buffer = GLBuffer::with_vertex(data, usage)?;

        Ok(self.push(gl_buffer))
    }

    fn new_index<T>(
        &mut self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Handle<Buffer>, RendererError> {
        let gl_buffer = GLBuffer::with_index(data, usage)?;

        Ok(self.push(gl_buffer))
    }
}

impl From<BufferUsage> for GLenum {
    fn from(usage: BufferUsage) -> Self {
        match usage {
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::StaticWrite => gl::STATIC_DRAW,
            BufferUsage::StaticReadWrite => gl::STATIC_DRAW | gl::STATIC_READ,
            BufferUsage::StaticCopy => gl::STATIC_DRAW,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::DynamicWrite => gl::DYNAMIC_DRAW,
            BufferUsage::DynamicReadWrite => gl::DYNAMIC_READ | gl::DYNAMIC_DRAW,
            BufferUsage::DynamicCopy => gl::DYNAMIC_READ,
            BufferUsage::StreamingRead => gl::STREAM_READ,
            BufferUsage::StreamingWrite => gl::STREAM_DRAW,
            BufferUsage::StreamingReadWrite => gl::STREAM_DRAW | gl::STREAM_READ,
            BufferUsage::StreamingCopy => gl::STREAM_COPY,
        }
    }
}

impl GLBuffer {
    pub(super) fn with_vertex<T>(data: &[T], usage: BufferUsage) -> Result<Self, RendererError> {
        Self::new(gl::ARRAY_BUFFER, data, usage)
    }

    pub(super) fn with_index<T>(data: &[T], usage: BufferUsage) -> Result<Self, RendererError> {
        Self::new(gl::ELEMENT_ARRAY_BUFFER, data, usage)
    }

    fn new<T>(kind: GLenum, data: &[T], usage: BufferUsage) -> Result<Self, RendererError> {
        let mut buffer = GLBuffer {
            kind,
            id: unsafe {
                let mut vbo = 0;
                gl::GenBuffers(1, &mut vbo as *mut GLuint);
                vbo
            },
        };

        buffer.set_data(data, usage)?;
        Ok(buffer)
    }

    pub(super) fn set_data<T>(
        &mut self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<(), RendererError> {
        self.bind();

        let size = std::mem::size_of::<T>() * data.len();

        let size = match size.try_into() {
            Ok(val) => val,
            Err(e) => {
                return Err(RendererError::ConversionError {
                    error: format!(
                        "Failed to convert Buffer usize({size}) into isize{}: {e}",
                        isize::MAX
                    ),
                })
            }
        };

        unsafe {
            gl::BufferData(
                self.kind,
                size,
                data.as_ptr() as *const std::ffi::c_void,
                usage.into(),
            );
        }

        Ok(())
    }

    pub(super) fn bind(&self) {
        unsafe { gl::BindBuffer(self.kind, self.id) }
    }
}

impl Drop for GLBuffer {
    fn drop(&mut self) {
        if self.id > 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
            }
        }
    }
}
