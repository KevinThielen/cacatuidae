use std::{cell::RefCell, fmt::Display};

use crate::{BufferAttributes, Handle, Renderer, RendererError};

use super::Context;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexLayout {}

impl VertexLayout {
    pub fn new<C: Context>(
        ctx: &mut Renderer<C>,
        buffer_attributes: &[BufferAttributes],
    ) -> Result<Handle<Self>, RendererError> {
        let mut vao = C::VertexLayout::new(ctx)?;
        for buffer_attr in buffer_attributes {
            if let Some(buffer) = ctx.buffers.get(buffer_attr.buffer) {
                vao.set_buffer_attributes(buffer, &buffer_attr.attributes, buffer_attr.offset)?;
            }
        }
        Ok(ctx.layouts.push(vao))
    }
}

pub trait CreateVertexLayout: Sized {
    type Buffer;

    fn new<C: Context>(ctx: &mut Renderer<C>) -> Result<Self, RendererError>;

    fn set_buffer_attributes(
        &mut self,
        buffer: &Self::Buffer,
        attributes: &[VertexAttribute],
        offset: usize,
    ) -> Result<(), RendererError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttributeSemantic {
    Position,
    UV(usize),
    Color(usize),
    Normals(usize),
    Tangent,
    Weights(usize),
    Joints(usize),
    Custom(VertexAttributeKind, usize),
}

impl Display for AttributeSemantic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            AttributeSemantic::Position => "Position".to_string(),
            AttributeSemantic::UV(n) => format!("UV[{n}]"),
            AttributeSemantic::Color(n) => format!("Color[{n}]"),
            AttributeSemantic::Normals(n) => format!("Normals[{n}]"),
            AttributeSemantic::Tangent => "Tangent".to_string(),
            AttributeSemantic::Weights(n) => format!("Weights[{n}]"),
            AttributeSemantic::Joints(n) => format!("Joints[{n}]"),
            AttributeSemantic::Custom(kind, n) => format!("Custom[{n}]: {kind}"),
        };

        write!(f, "{name}: Loc:{:?}", self.location())
    }
}

std::thread_local! {
static DEFAULT_LOCATIONS: RefCell<[Option<AttributeSemantic>; 16]> = RefCell::new([
    Some(AttributeSemantic::Position),
    Some(AttributeSemantic::UV(0)),
    Some(AttributeSemantic::UV(1)),
    Some(AttributeSemantic::UV(2)),
    Some(AttributeSemantic::UV(3)),
    Some(AttributeSemantic::Color(0)),
    Some(AttributeSemantic::Color(1)),
    Some(AttributeSemantic::Color(2)),
    Some(AttributeSemantic::Color(3)),
    Some(AttributeSemantic::Color(4)),
    Some(AttributeSemantic::Tangent),
    Some(AttributeSemantic::Normals(0)),
    Some(AttributeSemantic::Normals(1)),
    Some(AttributeSemantic::Normals(2)),
    Some(AttributeSemantic::Weights(3)),
    Some(AttributeSemantic::Joints(0)),
]);
}

impl AttributeSemantic {
    //TODO: replace with default kinds ref cell
    pub fn kind(&self) -> VertexAttributeKind {
        match self {
            AttributeSemantic::Position => VertexAttributeKind::Vec3,
            AttributeSemantic::UV(_) => VertexAttributeKind::Vec2,
            AttributeSemantic::Color(_) => VertexAttributeKind::Vec4,
            AttributeSemantic::Normals(_) => VertexAttributeKind::Vec3,
            AttributeSemantic::Weights(_) => VertexAttributeKind::Vec4,
            AttributeSemantic::Joints(_) => VertexAttributeKind::Vec4,
            AttributeSemantic::Custom(kind, _) => *kind,
            AttributeSemantic::Tangent => VertexAttributeKind::Vec3,
        }
    }

    //TODO: False for now, but once a "high performance" set of default semantic kinds is added,
    //this will allow us to reduce memory size for the attributes. For example, Normals don't need
    //32 bits per channel and could do with like 10.
    //Might even be better to tie it to the kind. But I don't believe this to become an actual issue
    //in the forseeable future, so I just yolo it now.
    pub fn normalized(&self) -> bool {
        false
    }

    pub fn location(&self) -> Option<u8> {
        DEFAULT_LOCATIONS.with(|f| {
            f.borrow()
                .iter()
                .enumerate()
                .find_map(|(index, attr)| match *attr {
                    Some(attr) if attr == *self => Some(index as u8),
                    _ => None,
                })
        })
    }

    pub fn set_default_locations(locations: [Option<AttributeSemantic>; 16]) {
        DEFAULT_LOCATIONS.with(|f| *f.borrow_mut() = locations)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VertexAttributeKind {
    F32,
    Vec2,
    Vec3,
    Vec4,
}

impl Display for VertexAttributeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VertexAttributeKind::F32 => write!(f, "F32"),
            VertexAttributeKind::Vec2 => write!(f, "Vec2"),
            VertexAttributeKind::Vec3 => write!(f, "Vec3"),
            VertexAttributeKind::Vec4 => write!(f, "Vec4"),
        }
    }
}

impl VertexAttributeKind {
    pub fn components(&self) -> u8 {
        match self {
            VertexAttributeKind::F32 => 1,
            VertexAttributeKind::Vec2 => 2,
            VertexAttributeKind::Vec3 => 3,
            VertexAttributeKind::Vec4 => 4,
        }
    }
    pub fn size(&self) -> usize {
        usize::from(self.components()) * std::mem::size_of::<f32>()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttribute {
    pub stride: usize,
    pub semantic: AttributeSemantic,
    pub normalized: bool,
    pub offset: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_location() {
        assert_eq!(AttributeSemantic::Position.location(), Some(0));
        assert_eq!(AttributeSemantic::UV(0).location(), Some(1));
        assert_eq!(AttributeSemantic::UV(10).location(), None);

        let mut new_locations = DEFAULT_LOCATIONS.with(|f| *f.borrow());
        new_locations[1] = Some(AttributeSemantic::UV(10));
        AttributeSemantic::set_default_locations(new_locations);

        assert_eq!(AttributeSemantic::Position.location(), Some(0));
        assert_eq!(AttributeSemantic::UV(0).location(), None);
        assert_eq!(AttributeSemantic::UV(10).location(), Some(1));
    }
}
