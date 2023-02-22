use std::{boxed, collections::HashMap};

use crate::{api, Result};

#[derive(Debug)]
pub struct Engine {
    property_table: HashMap<api::ProperyID, api::Property>,
    pending_properties_updates: Vec<(api::ProperyID, api::PropertyValue)>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            property_table: HashMap::new(),
            pending_properties_updates: Vec::new(),
        }
    }

    pub fn boxed() -> boxed::Box<Self> {
        boxed::Box::new(Self::new())
    }

    pub fn add_property(&mut self, property: api::Property) {
        self.property_table.insert(property.id().clone(), property);
    }

    pub fn append_property_update(&mut self, id: api::ProperyID, value: api::PropertyValue) {
        self.pending_properties_updates.push((id, value));
    }

    pub fn step(&mut self) -> Result<HashMap<api::ProperyID, api::Property>> {
        todo!("For each step, drain the pending property updates and update the scene. Return Copy of Property Table")
    }
}
