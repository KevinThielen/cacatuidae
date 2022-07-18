use std::fmt::Display;

use crate::{math, Handle, Renderer, RendererError};

use super::{Context, ShaderProgram, Uniform, UniformDescription};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Material {
    pub program: Handle<ShaderProgram>,
    pub(crate) data: Vec<u8>,
}

impl Material {
    pub fn new<C: Context>(
        ctx: &mut Renderer<C>,
        shader_program: Handle<ShaderProgram>,
        properties: &[MaterialProperty],
    ) -> Result<Handle<Self>, RendererError> {
        if let Some(program) = ctx.programs.get(shader_program) {
            let mut material = Material {
                program: shader_program,
                data: Vec::with_capacity(program.data_size()),
            };

            material.update(program.uniforms(), properties);

            Ok(ctx.materials.push(material))
        } else {
            Err(RendererError::ResourceNotFound {
                resource: "ShaderProgram: {shader_program}".to_string(),
            })
        }
    }
}

impl Material {
    pub(super) fn update(
        &mut self,
        uniforms: &[UniformDescription],
        properties: &[MaterialProperty],
    ) {
        for prop in properties {
            if let Some(uniform) = match prop.property {
                PropertyId::Name(name) => uniforms.iter().find(|uniform| uniform.name == name),
                PropertyId::Location(loc) => {
                    uniforms.iter().find(|uniform| uniform.location == loc)
                }
            } {
                match prop.value {
                    PropertyValue::F32(value) => {
                        value.iter().enumerate().for_each(|(index, v)| {
                            let bits = v.to_le_bytes();
                            let index = uniform.offset + index * 4;
                            self.data.splice(index..(index + 4), bits);
                        });
                    }
                };
            } else {
                log::warn!("Property {} not found in ShaderProgram", prop.property)
            }
        }
    }
}

pub enum PropertyId<'a> {
    Name(&'a str),
    Location(u32),
}

impl<'a> From<&'a str> for PropertyId<'a> {
    fn from(name: &'a str) -> Self {
        PropertyId::Name(name)
    }
}

impl<'a> From<u32> for PropertyId<'a> {
    fn from(location: u32) -> Self {
        PropertyId::Location(location)
    }
}
#[derive(Debug, PartialEq)]
pub enum PropertyValue<'a> {
    F32(&'a [f32]),
}

pub struct MaterialProperty<'a> {
    pub property: PropertyId<'a>,
    pub value: PropertyValue<'a>,
}

impl<'a> MaterialProperty<'a> {
    pub fn new<T: Into<PropertyId<'a>>, U: AsPropertyValue>(name: T, value: &'a U) -> Self {
        Self {
            property: name.into(),
            value: value.as_property_value(),
        }
    }
}

impl Display for PropertyId<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyId::Name(name) => write!(f, "Name: {name}"),
            PropertyId::Location(loc) => write!(f, "Location: {loc}"),
        }
    }
}

pub trait AsPropertyValue {
    fn as_property_value(&self) -> PropertyValue;
}

impl AsPropertyValue for f32 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(std::slice::from_ref(self))
    }
}
impl<const N: usize> AsPropertyValue for [f32; N] {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self)
    }
}

impl AsPropertyValue for math::Vec2 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [math::Vec2; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 2 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for math::Vec3 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [math::Vec3; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 3 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for math::Vec4 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [math::Vec4; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 4 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for math::Mat2 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [math::Mat2; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 4 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for math::Mat3 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [math::Mat3; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 9 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for math::Mat4 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [math::Mat4; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 16 * N) };
        PropertyValue::F32(data)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f32_prop_value() {
        let value = 10.0;
        let prop = value.as_property_value();

        assert_eq!(PropertyValue::F32(&[10.0]), prop);

        let values = [10.0, 11.0, 12.23214, 13.0];
        let prop = values.as_property_value();
        assert_eq!(PropertyValue::F32(&[10.0, 11.0, 12.23214, 13.0]), prop);
    }
    #[test]
    fn vec2_prop_value() {
        let value = math::vec2(10.0, 22.1234);
        let prop = value.as_property_value();

        assert_eq!(PropertyValue::F32(&[10.0, 22.1234]), prop);

        let values = [math::vec2(10.0, 11.0), math::vec2(12.0, 13.0)];
        let prop = values.as_property_value();
        assert_eq!(PropertyValue::F32(&[10.0, 11.0, 12.0, 13.0]), prop);
    }
    #[test]
    fn vec3_prop_value() {
        let value = math::vec3(10.0, 22.1234, 0.0012);
        let prop = value.as_property_value();

        assert_eq!(PropertyValue::F32(&[10.0, 22.1234, 0.0012]), prop);

        let values = [
            math::vec3(10.0, 11.0, 12.111),
            math::vec3(12.0, 13.0, 0.001),
        ];
        let prop = values.as_property_value();
        assert_eq!(
            PropertyValue::F32(&[10.0, 11.0, 12.111, 12.0, 13.0, 0.001]),
            prop
        );
    }
    #[test]
    fn vec4_prop_value() {
        let value = math::vec4(10.0, 22.1234, 11.0, 12.0);
        let prop = value.as_property_value();

        assert_eq!(PropertyValue::F32(&[10.0, 22.1234, 11.0, 12.0]), prop);

        let values = [
            math::vec4(10.0, 11.0, 17.0, 11.0),
            math::vec4(12.0, 13.0, 31.0, 21.0),
        ];
        let prop = values.as_property_value();
        assert_eq!(
            PropertyValue::F32(&[10.0, 11.0, 17.0, 11.0, 12.0, 13.0, 31.0, 21.0]),
            prop
        );
    }
    #[test]
    fn mat2_prop_value() {
        let value = math::mat2(math::vec2(10.0, 22.1234), math::vec2(11.0, 12.0));
        let prop = value.as_property_value();

        assert_eq!(PropertyValue::F32(&[10.0, 22.1234, 11.0, 12.0]), prop);

        let values = [value, value];
        let prop = values.as_property_value();
        assert_eq!(
            PropertyValue::F32(&[10.0, 22.1234, 11.0, 12.0, 10.0, 22.1234, 11.0, 12.0]),
            prop
        );
    }

    #[test]
    fn mat3_prop_value() {
        let value = math::mat3(
            math::vec3(10.0, 22.1234, 9.0),
            math::vec3(11.0, 12.0, 0.9),
            math::vec3(1.0, 2.0, 1.0),
        );
        let prop = value.as_property_value();

        assert_eq!(
            PropertyValue::F32(&[10.0, 22.1234, 9.0, 11.0, 12.0, 0.9, 1.0, 2.0, 1.0]),
            prop
        );

        let values = [value, value];
        let prop = values.as_property_value();
        assert_eq!(
            PropertyValue::F32(&[
                10.0, 22.1234, 9.0, 11.0, 12.0, 0.9, 1.0, 2.0, 1.0, 10.0, 22.1234, 9.0, 11.0, 12.0,
                0.9, 1.0, 2.0, 1.0
            ]),
            prop
        );
    }
    #[test]
    fn mat4_prop_value() {
        let value = math::mat4(
            math::vec4(10.0, 22.1234, 9.0, 4.0),
            math::vec4(11.0, 12.0, 0.9, 4.0),
            math::vec4(1.0, 2.0, 1.0, 4.0),
            math::vec4(4.0, 2.0, 1.0, 4.0),
        );
        let prop = value.as_property_value();

        assert_eq!(
            PropertyValue::F32(&[
                10.0, 22.1234, 9.0, 4.0, 11.0, 12.0, 0.9, 4.0, 1.0, 2.0, 1.0, 4.0, 4.0, 2.0, 1.0,
                4.0
            ]),
            prop
        );

        let values = [value, value];
        let prop = values.as_property_value();
        assert_eq!(
            PropertyValue::F32(&[
                10.0, 22.1234, 9.0, 4.0, 11.0, 12.0, 0.9, 4.0, 1.0, 2.0, 1.0, 4.0, 4.0, 2.0, 1.0,
                4.0, 10.0, 22.1234, 9.0, 4.0, 11.0, 12.0, 0.9, 4.0, 1.0, 2.0, 1.0, 4.0, 4.0, 2.0,
                1.0, 4.0,
            ]),
            prop
        );
    }
}
