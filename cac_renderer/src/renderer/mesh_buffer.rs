use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
pub enum BufferKind {
    Vertex,
    Index,
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
    Custom(AttributeKind, usize),
}

std::thread_local! {
static DEFAULT_LOCATIONS: RefCell<[Option<AttributeSemantic>; 16]> = RefCell::new([
    Some(AttributeSemantic::Position),
    Some(AttributeSemantic::UV(0)),
    Some(AttributeSemantic::UV(1)),
    Some(AttributeSemantic::UV(2)),
    Some(AttributeSemantic::UV(3)),
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
    None,
]);
}

impl AttributeSemantic {
    pub fn kind(&self) -> AttributeKind {
        match self {
            AttributeSemantic::Position => AttributeKind::Vec3,
            AttributeSemantic::UV(_) => AttributeKind::Vec2,
            AttributeSemantic::Color(_) => AttributeKind::Vec4,
            AttributeSemantic::Normals(_) => AttributeKind::Vec3,
            AttributeSemantic::Weights(_) => AttributeKind::Vec4,
            AttributeSemantic::Joints(_) => AttributeKind::Vec4,
            AttributeSemantic::Custom(kind, _) => *kind,
            AttributeSemantic::Tangent => AttributeKind::Vec3,
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
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttributeKind {
    F32,
    Vec2,
    Vec3,
    Vec4,
    Mat4,
}

impl AttributeKind {
    pub fn components(&self) -> u8 {
        match self {
            AttributeKind::F32 => 1,
            AttributeKind::Vec2 => 2,
            AttributeKind::Vec3 => 3,
            AttributeKind::Vec4 => 4,
            AttributeKind::Mat4 => 16,
        }
    }
    pub fn size(&self) -> usize {
        usize::from(self.components()) * std::mem::size_of::<f32>()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BufferAttribute {
    pub semantic: AttributeSemantic,
    pub normalized: bool,
    pub offset: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
    StaticRead,
    StaticWrite,
    StaticReadWrite,
    DynamicRead,
    DynamicWrite,
    DynamicReadWrite,
    StreaminRead,
    StreaminWrite,
    StreaminReadWrite,
}

#[derive(Clone, Debug)]
pub struct MeshBuffer<'a, T> {
    pub stride: usize,
    pub kind: BufferKind,
    pub attributes: Vec<BufferAttribute>,
    pub size: usize,
    pub vertex_count: u32,
    pub data: &'a [T],
    pub usage: BufferUsage,
    pub divisor: usize,
}

impl<'a, T> MeshBuffer<'a, T> {
    pub fn with_position(data: &'a [T]) -> Self {
        let kind = BufferKind::Vertex;

        let attributes = vec![BufferAttribute {
            semantic: AttributeSemantic::Position,
            normalized: false,
            offset: 0,
        }];

        let size = std::mem::size_of::<T>() * data.len();
        let vertex_count = (size
            / attributes
                .iter()
                .map(|attr| attr.semantic.kind().size())
                .sum::<usize>()) as u32;

        Self {
            stride: 0,
            kind,
            usage: BufferUsage::StaticRead,
            attributes,
            size,
            vertex_count,
            data,
            divisor: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn data_size() {
        let vertices: [(f32, f32, f32); 3] = [(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (-1.0, -1.0, -1.0)];

        let buffer = MeshBuffer::with_position(&vertices);

        assert_eq!(buffer.size, 36);
    }

    #[test]
    fn vertex_count() {
        let vertices = [(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (-1.0, -1.0, -1.0)];

        let buffer = MeshBuffer::with_position(&vertices);
        assert_eq!(buffer.vertex_count, 3);
    }

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
