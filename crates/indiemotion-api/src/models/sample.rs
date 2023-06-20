use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SampleProperty {
    pub name: String,
    pub value: super::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sample {
    properties: Vec<super::SampleProperty>,
}

impl Sample {
    pub fn new(properties: Vec<super::SampleProperty>) -> Self {
        Self { properties }
    }

    pub fn properties(&self) -> &Vec<super::SampleProperty> {
        &self.properties
    }
}
