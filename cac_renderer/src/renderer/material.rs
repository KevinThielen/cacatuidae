use std::fmt::Display;

use crate::Handle;

use super::{ShaderProgram, UniformDescription};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Material {}

pub struct MaterialData {
    pub(super) program: Handle<ShaderProgram>,
    pub(super) data: Vec<u8>,
}

impl MaterialData {
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

impl AsPropertyValue for crate::Vec2 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [crate::Vec2; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 2 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for crate::Vec3 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [crate::Vec3; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 3 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for crate::Vec4 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [crate::Vec4; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 4 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for crate::Mat2 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [crate::Mat2; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 4 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for crate::Mat3 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [crate::Mat3; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 9 * N) };
        PropertyValue::F32(data)
    }
}
impl AsPropertyValue for crate::Mat4 {
    fn as_property_value(&self) -> PropertyValue {
        PropertyValue::F32(self.as_ref())
    }
}
impl<const N: usize> AsPropertyValue for [crate::Mat4; N] {
    fn as_property_value(&self) -> PropertyValue {
        let data = unsafe { std::slice::from_raw_parts(self.as_ptr() as *const f32, 12 * N) };
        PropertyValue::F32(data)
    }
}
