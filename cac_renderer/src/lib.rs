//#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
mod color;
mod frame_timer;
pub use color::{Color32, Color8};
pub use frame_timer::FrameTimer;

mod error;
pub use error::RendererError;

mod renderer;
pub use renderer::{
    AttributeSemantic, Backend, Buffer, BufferAttributes, BufferData, BufferStorage, BufferUsage,
    ClearFlags, Material, MaterialProperty, Mesh, Primitive, ProgramStorage, PropertyId,
    PropertyValue, RenderTarget, Renderer, Shader, ShaderProgram, Texture, VertexAttribute,
    VertexAttributeKind, VertexLayout,
};

mod generation_vec;
pub use generation_vec::Handle;

pub mod math;
