use crate::{Error, Result};
use serde_derive::{Deserialize, Serialize};

type Vec4 = (f64, f64, f64, f64);
type Matrix44 = (Vec4, Vec4, Vec4, Vec4);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Float(f64),
    Vec3(Vec3),
    Vec4(Vec4),
    Matrix44(Matrix44),
}

impl Value {
    pub fn update(&mut self, other: &Self) -> Result<()> {
        match (self, other) {
            (Self::Float(ref mut this), Self::Float(them)) => {
                *this = *them;
                Ok(())
            }
            (Self::Vec3(ref mut this), Self::Vec3(them)) => {
                *this = them.clone();
                Ok(())
            }
            (Self::Vec4(ref mut this), Self::Vec4(them)) => {
                *this = *them;
                Ok(())
            }
            (Self::Matrix44(ref mut this), Self::Matrix44(them)) => {
                *this = *them;
                Ok(())
            }
            _ => Err(Error::InvalidValue("value has different type".into())),
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
