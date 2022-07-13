use crate::{Handle, RendererError, VertexAttribute};

pub trait BufferStorage {
    fn new_vertex<T>(
        &mut self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Handle<Buffer>, RendererError>;

    fn new_index<T>(
        &mut self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Handle<Buffer>, RendererError>;
}

#[derive(Copy, Clone, Debug)]
// TODO: Bitflags instead? But at least OpenGL uses separate constants for everything but Read |
// Write
pub enum BufferUsage {
    StaticRead,
    StaticWrite,
    StaticReadWrite,
    StaticCopy,
    DynamicRead,
    DynamicWrite,
    DynamicReadWrite,
    DynamicCopy,
    StreamingRead,
    StreamingWrite,
    StreamingReadWrite,
    StreamingCopy,
}

#[derive(Debug, Clone, Copy)]
pub enum BufferData<'a> {
    VertexF32(&'a [f32]),
    VertexU8(&'a [u8]),
    IndexU8(&'a [u8]),
    IndexU32(&'a [u32]),
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer {}

pub struct BufferAttributes<'a, T> {
    pub buffer: &'a T,
    pub attributes: &'a [VertexAttribute],
    pub offset: usize,
}
