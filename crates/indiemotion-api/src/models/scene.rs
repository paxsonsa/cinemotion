use std::{collections::HashMap, hash::Hash};

use serde_derive::{Deserialize, Serialize};

use crate::{Error, Result};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneGraph {
    pub name: String,
    pub objects: HashMap<u32, SceneObject>,
}

impl Default for SceneGraph {
    fn default() -> Self {
        let mut objects = HashMap::new();
        objects.insert(0, SceneObject::default());

        Self {
            name: "default".to_string(),
            objects,
        }
    }
}

impl SceneGraph {
    pub async fn add_object(&mut self, mut obj: SceneObject) -> Result<()> {
        match obj.id {
            Some(id) => {
                if self.objects.get(&id).is_none() {
                    return Err(Error::InvalidSceneObject(format!(
                        "object id {} does not exist",
                        id
                    )));
                }
                *self.objects.get_mut(&id).unwrap() = obj;
                Ok(())
            }
            None => {
                for existing in self.objects.values() {
                    if obj.name == existing.name {
                        return Err(Error::InvalidSceneObject(format!(
                            "object named {} already exists",
                            obj.name
                        )));
                    }
                }
                obj.id = Some(self.objects.len().try_into().unwrap());
                self.objects.insert(obj.id.unwrap(), obj);
                Ok(())
            }
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
