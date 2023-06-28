use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[cfg(test)]
#[path = "./value_test.rs"]
mod value_test;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    Float(f64),
    Vec3(Vec3),
    Vec4(Vec4),
    Matrix44(Matrix44),
}

impl Value {
    pub fn vec3() -> Self {
        Self::Vec3(Vec3::default())
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
                *this = *them;
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
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<(f64, f64, f64)> for Value {
    fn from(value: (f64, f64, f64)) -> Self {
        Self::Vec3(value.into())
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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

pub type Matrix44 = (
    (f64, f64, f64, f64),
    (f64, f64, f64, f64),
    (f64, f64, f64, f64),
    (f64, f64, f64, f64),
);
