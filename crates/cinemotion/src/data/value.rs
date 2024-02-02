use crate::{Error, Result};
use cinemotion_proto as proto;

#[derive(Debug, Clone, PartialEq)]
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

impl From<(f64, f64, f64, f64)> for Value {
    fn from(value: (f64, f64, f64, f64)) -> Self {
        Self::Vec4(value.into())
    }
}

impl From<[[f64; 4]; 4]> for Value {
    fn from(value: [[f64; 4]; 4]) -> Self {
        Self::Matrix44(value.into())
    }
}

impl From<Vec3> for Value {
    fn from(value: Vec3) -> Self {
        Self::Vec3(value)
    }
}

impl From<Vec4> for Value {
    fn from(value: Vec4) -> Self {
        Self::Vec4(value)
    }
}

impl From<Matrix44> for Value {
    fn from(value: Matrix44) -> Self {
        Self::Matrix44(value)
    }
}

impl From<proto::PropertyValue> for Value {
    fn from(prop_value: proto::PropertyValue) -> Self {
        match prop_value.value.unwrap() {
            proto::property_value::Value::FloatValue(value) => Self::Float(value),
            proto::property_value::Value::Vec3Value(value) => Self::Vec3(value.into()),
            proto::property_value::Value::Vec4Value(value) => Self::Vec4(value.into()),
            proto::property_value::Value::Matrix4x4Value(value) => Self::Matrix44(value.into()),
        }
    }
}

impl From<Value> for proto::PropertyValue {
    fn from(value: Value) -> Self {
        let mut prop_value = proto::PropertyValue::default();
        match value {
            Value::Float(value) => {
                prop_value.value = Some(proto::property_value::Value::FloatValue(value));
            }
            Value::Vec3(value) => {
                prop_value.value = Some(proto::property_value::Value::Vec3Value(value.into()));
            }
            Value::Vec4(value) => {
                prop_value.value = Some(proto::property_value::Value::Vec4Value(value.into()));
            }
            Value::Matrix44(value) => {
                prop_value.value = Some(proto::property_value::Value::Matrix4x4Value(value.into()));
            }
        }
        prop_value
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

impl From<proto::Vec3> for Vec3 {
    fn from(value: proto::Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vec3> for proto::Vec3 {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
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

impl From<proto::Vec4> for Vec4 {
    fn from(value: proto::Vec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<Vec4> for proto::Vec4 {
    fn from(value: Vec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
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

impl From<Matrix44> for proto::Matrix4x4 {
    fn from(value: Matrix44) -> Self {
        Self {
            row0: Some(value.row0.into()),
            row1: Some(value.row1.into()),
            row2: Some(value.row2.into()),
            row3: Some(value.row3.into()),
        }
    }
}

impl From<proto::Matrix4x4> for Matrix44 {
    fn from(value: proto::Matrix4x4) -> Self {
        Self {
            row0: value.row0.unwrap().into(),
            row1: value.row1.unwrap().into(),
            row2: value.row2.unwrap().into(),
            row3: value.row3.unwrap().into(),
        }
    }
}
