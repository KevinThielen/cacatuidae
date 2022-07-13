use std::{cell::RefCell, fmt::Display};

use crate::{BufferAttributes, Handle, RendererError};

pub trait LayoutStorage {
    type Buffer;

    fn new(
        &mut self,
        buffer_attributes: &[BufferAttributes<Self::Buffer>],
    ) -> Result<Handle<VertexLayout>, RendererError>;
}
#[derive(Debug, Clone, Copy)]
pub struct VertexLayout {}

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
            AttributeSemantic::Custom(kind, n) => format!("Custom[{n}]"),
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

    pub fn update_default_location(semantic: Option<AttributeSemantic>, index: usize) {
        if index >= 16 {
            log::error!("Locations for Attribute Semantics must be < 16, tried to update location {{{index}}}");
        } else {
            todo!()
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VertexAttributeKind {
    F32,
    Vec2,
    Vec3,
    Vec4,
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
