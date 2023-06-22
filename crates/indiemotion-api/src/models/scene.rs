use std::{collections::HashMap, fmt::Display, hash::Hash};

use serde::{Deserialize, Serialize};

use crate::{Error, Name, Result};

use super::*;

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Clone)]
pub struct ObjectName(String);

impl Display for ObjectName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Hash for ObjectName {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl From<String> for ObjectName {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&str> for ObjectName {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scene {
    pub name: String,
    objects: HashMap<Name, SceneObject>,
}

impl Default for Scene {
    fn default() -> Self {
        let mut objects = HashMap::new();
        objects.insert("default".into(), SceneObject::default());

        Self {
            name: "default".to_string(),
            objects,
        }
    }
}

impl Scene {
    pub fn objects(&self) -> &HashMap<Name, SceneObject> {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut HashMap<Name, SceneObject> {
        &mut self.objects
    }

    pub fn object(&self, name: &Name) -> Option<&SceneObject> {
        self.objects.get(name)
    }

    pub async fn add_object(&mut self, obj: SceneObject) -> Result<()> {
        match self.object(&obj.name) {
            Some(_) => Err(Error::InvalidSceneObject(format!(
                "object named {} already exists",
                obj.name
            ))),
            None => {
                self.objects.insert(obj.name.clone(), obj);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneObject {
    name: Name,
    properties: HashMap<String, ObjectProperty>,
}

impl SceneObject {
    pub fn new(name: Name, properties: Vec<ObjectProperty>) -> Self {
        let properties = properties
            .into_iter()
            .map(|x| (x.name().to_string(), x))
            .collect();

        Self { name, properties }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn properties(&self) -> &HashMap<String, ObjectProperty> {
        &self.properties
    }

    pub fn properties_mut(&mut self) -> &mut HashMap<String, ObjectProperty> {
        &mut self.properties
    }

    pub fn property(&self, name: &str) -> Option<&ObjectProperty> {
        self.properties.get(name)
    }
}

impl Default for SceneObject {
    fn default() -> Self {
        Self::new(
            "default".into(),
            vec![
                ObjectProperty::new_vec3("translate".into()),
                ObjectProperty::new_vec3("orientation".into()),
                ObjectProperty::new_vec3("velocity".into()),
            ],
        )
    }
}
