use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Attribute {
    name: Name,
    value: AttributeValue,
    default_value: AttributeValue,
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
                (this.row0.x, this.row0.y, this.row0.z, this.row0.w) =
                    (them.row0.x, them.row0.y, them.row0.z, them.row0.w);
                (this.row1.x, this.row1.y, this.row1.z, this.row1.w) =
                    (them.row1.x, them.row1.y, them.row1.z, them.row1.w);
                (this.row2.x, this.row2.y, this.row2.z, this.row2.w) =
                    (them.row2.x, them.row2.y, them.row2.z, them.row2.w);
                (this.row3.x, this.row3.y, this.row3.z, this.row3.w) =
                    (them.row3.x, them.row3.y, them.row3.z, them.row3.w);
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

impl From<[[f64; 4]; 4]> for AttributeValue {
    fn from(value: [[f64; 4]; 4]) -> Self {
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix44 {
    pub row0: Vec4,
    pub row1: Vec4,
    pub row2: Vec4,
    pub row3: Vec4,
}

impl From<[[f64; 4]; 4]> for Matrix44 {
    fn from(value: [[f64; 4]; 4]) -> Self {
        Self {
            row0: value[0].into(),
            row1: value[1].into(),
            row2: value[2].into(),
            row3: value[3].into(),
        }
    }
}
