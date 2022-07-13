use super::{vertex_layout::VertexLayout, Handle};

#[derive(Debug, Copy, Clone)]
pub enum Primitive {
    Triangles,
    TriangleStrip,
    Lines,
    LineStrip,
    Points,
}

#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    pub vertex_layout: Handle<VertexLayout>,
    pub start_index: usize,
    pub count: u32,
    pub primitive: Primitive,
}
