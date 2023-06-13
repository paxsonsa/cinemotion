use serde_derive::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneGraph {
    pub name: String,
    pub objects: Vec<SceneObject>,
}

impl Default for SceneGraph {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            objects: vec![SceneObject::default()],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneObject {
    pub id: Option<u32>,
    pub name: String,
    pub attributes: Vec<Attribute>,
}

impl Default for SceneObject {
    fn default() -> Self {
        Self {
            id: Some(0),
            name: "default".to_string(),
            attributes: vec![
                Attribute::new_vec3("translate"),
                Attribute::new_vec3("orientation"),
                Attribute::new_vec3("velocity"),
            ],
        }
    }
}
