use crate::{Result, Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttrName(String);

#[derive(Debug, Clone, PartialEq)]
pub enum AttrValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl AttrValue {
    pub fn variant_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(_), Self::String(_)) => true,
            (Self::Number(_), Self::Number(_)) =>  true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            _ => false
        }
    }
}

pub struct Attribute(AttrName, AttrValue);

impl Attribute {
    pub fn new(name: AttrName, value: AttrValue) -> Self {
        Self(name, value)
    }

    pub fn name(&self) -> &AttrName {
        &self.0
    }

    pub fn value(&self) -> AttrValue {
        self.1.clone()
    }

    pub fn set_value(&mut self, value: AttrValue) -> Result<()> {
        if !self.1.variant_eq(&value) {
            return Err(Error::AttributeTypeMismatch);
        }
        self.1 = value;
        Ok(())
    }
}
    