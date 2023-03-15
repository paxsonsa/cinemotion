#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ComponentID {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Vec3f(f64, f64, f64),
    Vec3i(i32, i32, i32),
}
