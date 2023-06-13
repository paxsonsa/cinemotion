use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

impl Attribute {
    pub fn new_vec3(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: AttributeValue::Vec3((0.0, 0.0, 0.0)),
        }
    }
}

type Vec3 = (f64, f64, f64);
type Vec4 = (f64, f64, f64, f64);
type Matrix44 = (Vec4, Vec4, Vec4, Vec4);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AttributeValue {
    Float(f64),
    Vec3(Vec3),
    Matrix44(Matrix44),
}
