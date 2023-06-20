use serde_derive::{Deserialize, Serialize};

use super::value::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObjectProperty {
    name: String,
    value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    binding: Option<PropertyBinding>,
}

impl ObjectProperty {
    pub fn new(name: String, value: Value, binding: Option<PropertyBinding>) -> Self {
        Self {
            name,
            value,
            binding,
        }
    }

    pub fn new_vec3(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: (0.0, 0.0, 0.0).into(),
            binding: None,
        }
    }

    pub fn as_vec3(&self) -> Option<&Vec3> {
        match &self.value {
            Value::Vec3(vec3) => Some(vec3),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn binding(&self) -> Option<&PropertyBinding> {
        self.binding.as_ref()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyBinding {
    pub namespace: String,
    pub property: String,
}

impl From<&str> for PropertyBinding {
    fn from(address: &str) -> Self {
        let mut parts = address.splitn(2, '.');
        let namespace = parts.next().unwrap_or("").to_string();
        let id = parts.next().unwrap_or("").to_string();
        Self {
            namespace,
            property: id,
        }
    }
}

impl From<String> for PropertyBinding {
    fn from(address: String) -> Self {
        address.as_str().into()
    }
}
