use crate::{AttributeSemantic, Handle, Renderer, RendererError, VertexAttribute};

use super::Context;

pub trait BufferStorage<C: Context> {
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

    fn push(&mut self, buffer: C::Buffer) -> Handle<Buffer>;
}

pub trait CreateBuffer: Sized {
    fn with_vertex<T>(data: &[T], usage: BufferUsage) -> Result<Self, RendererError>;
    fn with_index<T>(data: &[T], usage: BufferUsage) -> Result<Self, RendererError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Buffer {}

impl Buffer {
    pub fn with_vertex<T, C: Context>(
        ctx: &mut Renderer<C>,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Handle<Buffer>, RendererError> {
        let buffer = C::Buffer::with_vertex(data, usage)?;
        Ok(ctx.buffers.push(buffer))
    }

    pub fn with_index<T, C: Context>(
        ctx: &mut Renderer<C>,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Handle<Buffer>, RendererError> {
        let buffer = C::Buffer::with_index(data, usage)?;
        Ok(ctx.buffers.push(buffer))
    }
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

pub struct BufferAttributes {
    pub buffer: Handle<Buffer>,
    pub attributes: Vec<VertexAttribute>,
    pub offset: usize,
}

impl BufferAttributes {
    pub fn with_index(buffer: Handle<Buffer>, buffer_offset: usize) -> Self {
        Self {
            buffer,
            attributes: Vec::new(),
            offset: buffer_offset,
        }
    }
    pub fn with_semantics(
        buffer: Handle<Buffer>,
        buffer_offset: usize,
        semantics: &[AttributeSemantic],
    ) -> Self {
        let stride = semantics
            .iter()
            .map(|semantics| semantics.kind().size())
            .sum();

        let mut offset = 0;
        let attributes = semantics
            .iter()
            .map(|semantic| -> VertexAttribute {
                let attr = VertexAttribute {
                    stride,
                    semantic: *semantic,
                    normalized: semantic.normalized(),
                    offset,
                };
                offset += semantic.kind().size();
                attr
            })
            .collect();

        BufferAttributes {
            buffer,
            attributes,
            offset: buffer_offset,
        }
    }
}
