#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
mod color;
mod error;
mod renderer;

pub use color::{Color32, Color8};
pub use renderer::{ClearFlags, RenderTarget, Renderer};
