#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AttributeID {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Vec3f(f64, f64, f64),
    Vec3i(i32, i32, i32),
}
