use crate::Name;

#[derive(Default, Debug, Clone)]
pub struct Context {
    pub uid: usize,
    pub name: Option<Name>,
}
