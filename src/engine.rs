use std::{boxed, collections::HashMap};

use crate::{api, Error, Result};

#[cfg(test)]
#[path = "./engine_test.rs"]
mod engine_test;

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

    /// Add a property to the engine.
    ///
    /// If the property already exists, it will be overwritten.
    pub fn add_property(&mut self, property: api::Property) {
        self.property_table.insert(property.id().clone(), property);
    }

    pub fn append_property_update(
        &mut self,
        id: api::ProperyID,
        value: api::PropertyValue,
    ) -> Result<()> {
        let Some(property) = self.property_table.get(&id) else {
            return Err(Error::PropertyUpdateError(id, "property not found"));
        };

        if !property.value().is_compatible(&value) {
            return Err(Error::PropertyUpdateError(
                id,
                "attempted to change value type",
            ));
        }

        self.pending_properties_updates.push((id, value));
        Ok(())
    }

    pub fn step(&mut self) -> Result<HashMap<api::ProperyID, api::Property>> {
        let mut updates = self.pending_properties_updates.drain(..);
        let mut updated_properties = HashMap::new();
        while let Some((id, value)) = updates.next() {
            let Some(property) = self.property_table.get_mut(&id) else {
                continue;
            };
            property.set_value(value)?;
            updated_properties.insert(id, property.clone());
        }

        Ok(updated_properties)
    }

    pub fn reset(&mut self) {
        for (_, property) in self.property_table.iter_mut() {
            property.reset_value();
        }
        self.pending_properties_updates.drain(..);
    }
}
