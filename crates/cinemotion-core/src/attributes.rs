use crate::prelude::*;

use bevy_ecs::prelude::Component;
use std::{collections::HashMap, ops::Deref};

#[derive(Component, Debug)]
pub struct AttributeMap(HashMap<Name, Attribute>);

impl AttributeMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, name: &Name) -> Option<&Attribute> {
        self.0.get(name)
    }

    pub fn insert(&mut self, attribute: Attribute) {
        self.0.insert(attribute.name.clone(), attribute);
    }
}

impl Deref for AttributeMap {
    type Target = HashMap<Name, Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<Name, Attribute>> for AttributeMap {
    fn from(value: HashMap<Name, Attribute>) -> Self {
        Self(value)
    }
}

#[derive(Component)]
pub struct AttributeLinkMap(HashMap<Name, Attribute>);

impl AttributeLinkMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    name: Name,
    value: AttributeValue,
    default_value: AttributeValue,
}

#[derive(Clone, Debug)]
pub struct AttributeID(u32, Name);

impl AttributeID {
    pub fn new(entity_id: u32, name: Name) -> Self {
        Self(entity_id, name)
    }
}

#[derive(Debug)]
pub struct AttributeLink {
    source: AttributeID,
    attribute: Name,
}

impl AttributeLink {
    pub fn new<N: Into<Name>>(source: AttributeID, attribute: N) -> Self {
        Self {
            source,
            attribute: attribute.into(),
        }
    }

    pub fn mapped<N: Into<Name>>(source_id: u32, attribute: N) -> Self {
        let attribute: Name = attribute.into();
        Self {
            source: AttributeID::new(source_id, attribute.clone()),
            attribute,
        }
    }

    pub fn source(&self) -> &AttributeID {
        &self.source
    }

    pub fn attribute(&self) -> Name {
        self.attribute.clone()
    }
}

impl Attribute {
    pub fn new<N: Into<Name>>(name: N, value: AttributeValue) -> Self {
        Self {
            name: name.into(),
            value: value.clone(),
            default_value: value.clone(),
        }
    }

    pub fn new_vec3<N: Into<Name>>(name: N) -> Self {
        Self {
            name: name.into(),
            value: AttributeValue::vec3(),
            default_value: AttributeValue::vec3(),
        }
    }

    pub fn new_matrix44<N: Into<Name>>(name: N) -> Self {
        Self {
            name: name.into(),
            value: AttributeValue::matrix44(),
            default_value: AttributeValue::matrix44(),
        }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
    Float(f64),
    Vec3(Vec3),
    Vec4(Vec4),
    Matrix44(Matrix44),
}

impl AttributeValue {
    pub fn vec3() -> Self {
        Self::Vec3(Vec3::default())
    }

    pub fn matrix44() -> Self {
        Self::Matrix44(Matrix44::default())
    }

    pub fn update(&mut self, other: &Self) -> Result<()> {
        match (self, other) {
            (Self::Float(ref mut this), Self::Float(them)) => {
                *this = *them;
                Ok(())
            }
            (Self::Vec3(ref mut this), Self::Vec3(them)) => {
                (this.x, this.y, this.z) = (them.x, them.y, them.z);
                Ok(())
            }
            (Self::Vec4(ref mut this), Self::Vec4(them)) => {
                (this.x, this.y, this.z, this.w) = (them.x, them.y, them.z, them.w);
                Ok(())
            }
            (Self::Matrix44(ref mut this), Self::Matrix44(them)) => {
                this.data = them.data;
                Ok(())
            }
            _ => Err(Error::InvalidValue("value has different type".into())),
        }
    }

    pub fn as_f64(&self) -> Option<&f64> {
        match self {
            Self::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_vec3(&self) -> Option<&Vec3> {
        match self {
            Self::Vec3(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_matrix44(&self) -> Option<&Matrix44> {
        match self {
            Self::Matrix44(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_matrix44_mut(&mut self) -> Option<&mut Matrix44> {
        match self {
            Self::Matrix44(value) => Some(value),
            _ => None,
        }
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<(f64, f64, f64)> for AttributeValue {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::Vec3(value.into())
    }
}

impl From<(f64, f64, f64, f64)> for AttributeValue {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self::Vec4(value.into())
    }
}

impl From<[f64; 16]> for AttributeValue {
    fn from(value: [f64; 16]) -> Self {
        Self::Matrix44(value.into())
    }
}

impl From<Vec3> for AttributeValue {
    fn from(value: Vec3) -> Self {
        Self::Vec3(value)
    }
}

impl From<Vec4> for AttributeValue {
    fn from(value: Vec4) -> Self {
        Self::Vec4(value)
    }
}

impl From<Matrix44> for AttributeValue {
    fn from(value: Matrix44) -> Self {
        Self::Matrix44(value)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for (f64, f64, f64) {
    fn from(vec3: Vec3) -> Self {
        (vec3.x, vec3.y, vec3.z)
    }
}

impl std::cmp::PartialEq<(f64, f64, f64)> for Vec3 {
    fn eq(&self, other: &(f64, f64, f64)) -> bool {
        (self.x, self.y, self.z) == *other
    }
}

impl std::cmp::PartialEq<(f64, f64, f64)> for &Vec3 {
    fn eq(&self, other: &(f64, f64, f64)) -> bool {
        (self.x, self.y, self.z) == *other
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<[f64; 4]> for Vec4 {
    fn from(value: [f64; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}

impl From<(f64, f64, f64, f64)> for Vec4 {
    fn from((x, y, z, w): (f64, f64, f64, f64)) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Vec4> for (f64, f64, f64, f64) {
    fn from(vec4: Vec4) -> Self {
        (vec4.x, vec4.y, vec4.z, vec4.w)
    }
}

impl std::cmp::PartialEq<(f64, f64, f64, f64)> for Vec4 {
    fn eq(&self, other: &(f64, f64, f64, f64)) -> bool {
        (self.x, self.y, self.z, self.w) == *other
    }
}

impl std::cmp::PartialEq<(f64, f64, f64)> for &Vec4 {
    fn eq(&self, other: &(f64, f64, f64)) -> bool {
        (self.x, self.y, self.z) == *other
    }
}

// Represents a 4x4 double precision matrix.
//
// The matrix is represented a column major where each sub-tuple
// repsents a column.
//
#[derive(Debug, Clone, Copy, PartialEq)]
struct Matrix44 {
    data: [f64; 16], // Column-major storage
}

impl Matrix44 {
    pub fn new() -> Self {
        Self { data: [0.0; 16] }
    }

    pub fn identity() -> Self {
        let mut matrix = Self::new();
        matrix.data[0] = 1.0;
        matrix.data[5] = 1.0;
        matrix.data[10] = 1.0;
        matrix.data[15] = 1.0;
        matrix
    }

    pub fn get(&self, x: usize, y: usize) -> Option<f64> {
        if x < 4 && y < 4 {
            Some(self.data[y * 4 + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: f64) {
        if x < 4 && y < 4 {
            self.data[y * 4 + x] = value;
        }
    }
}

impl Default for Matrix44 {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<[f64; 16]> for Matrix44 {
    fn from(value: [f64; 16]) -> Self {
        Self { data: value }
    }
}
