use std::collections::HashMap;

use uuid::Uuid;

use crate::api;
use crate::engine::Engine;

#[cfg(test)]
#[path = "./runtime_test.rs"]
mod runtime_test;

pub struct Runtime {
    state: api::SessionState,
    clients: HashMap<Uuid, api::Client>,
    attributes: HashMap<api::AttrName, api::Attribute>,
    engine: Box<Engine>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            state: api::SessionState::default(),
            clients: HashMap::new(),
            attributes: HashMap::new(),
            engine: Engine::boxed(),
        }
    }

    pub fn add_client(&mut self, client: api::Client) {
        self.clients.insert(client.id, client);
        self.report_client_update();
    }

    pub fn remove_client(&mut self, id: Uuid) {
        self.clients.remove(&id);
        self.report_client_update();
    }

    fn report_client_update(&self) {
        for client in self.clients.values() {
            client.relay.report_client_update(&self.clients);
        }
    }

    pub fn update_attribute(&mut self, name: api::AttrName, value: api::AttrValue) -> api::Result<()> {

        match self.attributes.get_mut(&name) {
            Some(attr) => {
                attr.set_value(value)?;
            }
            None => {
                let attr = api::Attribute::new(name.clone(), value);
                self.attributes.insert(name.clone(), attr);
            }
        }

        for client in self.clients.values_mut() {
            client.relay.report_attribute_updates(&self.attributes);
        }
        Ok(())
    }

    pub fn update_mode(&mut self, mode: api::SessionMode) {
        if self.state.mode.variant_eq(&mode) {
            return;
        }
        self.state.mode = mode;
        self.report_session_update();
    }

    fn report_session_update(&self) {
        for client in self.clients.values() {
            client.relay.report_session_update(&self.state);
        }
    }

    pub fn add_property(&mut self, id: api::ProperyID, default_value: api::PropertyValue) {
        let prop = api::Property::new_with_id(id, default_value);
        self.engine.add_property(prop);
    }

    pub fn update_property(&mut self, id: api::ProperyID, value: api::PropertyValue) {
        self.engine.append_property_update(id, value);
        self.report_property_update();
    }
}