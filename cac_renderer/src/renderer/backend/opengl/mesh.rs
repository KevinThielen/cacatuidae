//use gl::types::{GLenum, GLuint};

//use crate::Primitive;

//impl From<Primitive> for GLenum {
//fn from(primitive: Primitive) -> Self {
//match primitive {
//Primitive::Triangles => gl::TRIANGLES,
//Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
//}
//}
//}

//struct BufferObject {}

//enum Indices {
//U16(u32),
//U32(u32),
//}

//pub(super) struct Mesh {
//vao: GLuint,
//buffers: Vec<BufferObject>,
//primitive: GLenum,
//vertex_count: u32,
//indices: Option<Indices>,
//}

//impl Drop for Mesh {
//fn drop(&mut self) {
//unsafe {
//gl::DeleteVertexArrays(1, &self.vao);
//}
//}
//}

//impl Mesh {
//pub(super) fn new(primitive: Primitive, mesh_buffers: &[MeshBuffer]) -> Self {
//let gl_mesh = Self {
//vao: unsafe {
//let mut vao = 0;
//gl::GenVertexArrays(1, &mut vao);
//vao
//},
//buffers: Vec::with_capacity(mesh_buffers.len()),
//primitive: primitive.into(),
//vertex_count: 0,
//indices: None,
//};

//gl_mesh.bind();

//mesh_buffers.iter()

//gl_mesh

//}

//pub(super) fn bind(&self) {
//unsafe {
//gl::BindVertexArray(self.vao);
//}
//}
//}
