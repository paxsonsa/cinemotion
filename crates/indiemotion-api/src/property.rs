use crate::{Error, Result};
use std::fmt::{Display, Formatter};

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

impl Display for ProperyID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.namespace, self.name)
    }
}

impl TryFrom<&str> for ProperyID {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        let mut split = s.split('.');
        let namespace = split.next().expect("Invalid ProperyID");
        let name = split.next().expect("Invalid ProperyID");

        if let Some(_) = split.next() {
            Err(Error::InvalidProperyID(s.to_string()))?;
        }

        Ok(Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    Float(f64),
    Int(i64),
    Boolean(bool),
}

impl PropertyValue {
    pub fn is_compatible(&self, other: &Self) -> bool {
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
            id: ProperyID { namespace, name },
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
        if !self.value.is_compatible(&value) {
            return Err(Error::PropertyTypeMismatch);
        }
        self.value = value;
        Ok(())
    }

    pub fn reset_value(&mut self) {
        self.value = self.default_value.clone();
    }

    pub fn set_default_value(&mut self, value: PropertyValue) -> Result<()> {
        if !self.default_value.is_compatible(&value) {
            return Err(Error::PropertyTypeMismatch);
        }
        self.default_value = value;
        Ok(())
    }
}
