#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
mod color;
pub use color::{Color32, Color8};

mod error;
pub use error::RendererError;

mod renderer;
pub use renderer::{ClearFlags, Mesh, RenderTarget, Renderer};

mod generation_vec;
pub use generation_vec::Handle;
