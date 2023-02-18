use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    id: ProperyID,
    value: PropertyValue,
    default_value: PropertyValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProperyID {
    namespace: String,
    name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    Float(f64),
    Int(i64),
    Boolean(bool),
}

impl PropertyValue {
    pub fn variant_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(_), Self::Float(_)) => true,
            (Self::Int(_), Self::Int(_)) => true,
            (Self::Boolean(_), Self::Boolean(_)) => true,
            _ => false,
        }
    }
}

impl Property {
    pub fn new(namespace: String, name: String, value: PropertyValue) -> Self {
        Self {
            id: ProperyID {
                namespace,
                name,
            },
            default_value: value.clone(),
            value,
        }
    }

    pub fn new_with_id(id: ProperyID, value: PropertyValue) -> Self {
        Self {
            id: id,
            default_value: value.clone(),
            value: value,
        }
    }

    pub fn id(&self) -> &ProperyID {
        &self.id
    }

    pub fn value(&self) -> &PropertyValue {
        &self.value
    }

    pub fn set_value(&mut self, value: PropertyValue) -> Result<()> {
        if !self.value.variant_eq(&value) {
            return Err(Error::PropertyTypeMismatch);
        }
        self.value = value;
        Ok(())
    }

    pub fn reset_value(&mut self) {
        self.value = self.default_value.clone();
    }

    pub fn set_default_value(&mut self, value: PropertyValue) -> Result<()> {
        if !self.default_value.variant_eq(&value) {
            return Err(Error::PropertyTypeMismatch);
        }
        self.default_value = value;
        Ok(())
    }

}