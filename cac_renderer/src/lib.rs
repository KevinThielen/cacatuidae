#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
// Compile time evaluation of the f32 functions(mainly just the color stuff) seems fine, even if
// they could potentially result in different output compared to runtime.
#![feature(const_fn_floating_point_arithmetic)]

mod backend;
mod color;
mod error;
mod mesh;
mod renderer;

pub use color::Color8;
pub use mesh::Mesh;
pub use renderer::Renderer;
