use std::{boxed, collections::HashMap};

use crate::api;

pub struct Engine {
    property_table: HashMap<api::ProperyID, api::Property>,
    pending_properties_updates: Vec<api::Property>,
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
}