use std::{boxed, collections::HashMap};

use crate::{api, Error, Result};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(test)]
#[path = "./engine_test.rs"]
mod engine_test;

#[derive(Debug)]
pub struct Engine {
    property_table: HashMap<api::ProperyID, api::Property>,
    update_queue: Arc<Mutex<Vec<(api::ProperyID, api::PropertyValue)>>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            property_table: HashMap::new(),
            update_queue: Default::default(),
        }
    }

    pub fn boxed() -> boxed::Box<Self> {
        boxed::Box::new(Self::new())
    }

    pub fn properties(&self) -> Vec<api::Property> {
        self.property_table.values().cloned().collect()
    }

    /// Add a property to the engine.
    ///
    /// If the property already exists, it will be overwritten.
    pub fn add_property(&mut self, property: api::Property) {
        self.property_table.insert(property.id().clone(), property);
    }

    pub fn remove_property(&mut self, id: &api::ProperyID) {
        self.property_table.remove(id);
    }

    pub async fn append_property_update(
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

        self.update_queue.lock().await.push((id, value));
        Ok(())
    }

    pub async fn step(&mut self) -> Result<HashMap<api::ProperyID, api::PropertyValue>> {
        let mut queue = self.update_queue.lock().await;
        let updates: Vec<(api::ProperyID, api::PropertyValue)> = queue.drain(..).collect();
        drop(queue);
        let mut updated_properties = HashMap::new();
        for (id, value) in updates {
            let Some(property) = self.property_table.get_mut(&id) else {
                continue;
            };
            property.set_value(value)?;
            updated_properties.insert(id, property.value().clone());
        }

        Ok(updated_properties)
    }

    pub fn reset(&mut self) {
        for (_, property) in self.property_table.iter_mut() {
            property.reset_value();
        }
        self.update_queue = Default::default();
    }
}
