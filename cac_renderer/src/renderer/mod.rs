mod backend;

pub use backend::{Backend, Context};

mod mesh;
pub use mesh::{Mesh, Primitive};

mod render_target;
pub use render_target::{ClearFlags, RenderTarget};

mod shader;
pub use shader::{ProgramStorage, Shader, ShaderProgram, ShaderStorage};

mod buffer;
pub use buffer::{Buffer, BufferAttributes, BufferData, BufferStorage, BufferUsage};

mod vertex_layout;
pub use vertex_layout::{
    AttributeSemantic, LayoutStorage, VertexAttribute, VertexAttributeKind, VertexLayout,
};

use crate::Handle;

/// Renderer abstraction
///
/// Provides a high level API for the graphics backends. The backend can be chosen dynamically,
/// depending on the supported features, version and graphics device capabilities. See
/// [StaticRenderer] if you want to use a compile-time dependent one to avoid the boxing.
///
/// [Shader]s, [Mesh]es, [Texture]s can be created on the graphics device, while the renderer returns handles to them.
/// The user is responsible to releasing them with the handles. It is recommended to
/// keep track of the Handles with a human readable constant to associate them with a name or id.
/// The Renderer sorts and batches draw calls.

pub struct Renderer<T: Context> {
    context: T::Context,
    pub buffers: T::BufferStorage,
    pub layouts: T::LayoutStorage,
    pub shaders: T::ShaderStorage,
    pub programs: T::ProgramStorage,
}

impl<T: Context> Renderer<T> {}

pub trait GraphicsStorage<K: Copy, V> {
    fn get(&self, handle: Handle<K>) -> Option<&V>;
    fn get_mut(&mut self, handle: Handle<K>) -> Option<&mut V>;
}
