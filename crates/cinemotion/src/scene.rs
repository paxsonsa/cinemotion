use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    data::{PropertyState, Value},
    name, Name, Result,
};

/// Represents the currently loaded scene in the system.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Scene {
    /// The name of the scene.
    pub name: Name,

    /// The objects in the scene.
    objects: HashMap<Name, SceneObject>,
}

/// Create the default scene.
///
/// The default scene contains the default object and is named "default".
///
impl Default for Scene {
    fn default() -> Self {
        let mut objects = HashMap::new();
        objects.insert("default".into(), SceneObject::default());

        Self {
            name: name!("default"),
            objects,
        }
    }
}

impl Scene {
    /// Get a reference to all the objects in the scene.
    pub fn objects(&self) -> &HashMap<Name, SceneObject> {
        &self.objects
    }

    /// Get a mutable reference to all the objects in the scene.
    pub fn objects_mut(&mut self) -> &mut HashMap<Name, SceneObject> {
        &mut self.objects
    }

    /// Get a reference to the object with the given name.
    pub fn object(&self, name: &Name) -> Option<&SceneObject> {
        self.objects.get(name)
    }

    /// Add a new object to the scene.
    ///
    /// This will error with `InvalidSceneObject` if an object with the same name already exists.
    ///
    pub async fn add_object(&mut self, obj: SceneObject) -> Result<()> {
        let _ = self.objects.insert(obj.name.clone(), obj);
        Ok(())
    }
}

/// Represents an object in the scene graph that can be animated but the controllers.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SceneObject {
    /// A unique name for the scene object.
    name: Name,
    /// A map of property names and property states.
    properties: HashMap<Name, PropertyState>,
}

impl SceneObject {
    /// Create a new scene object with the given name and properties.
    pub fn new(name: Name, properties: HashMap<Name, PropertyState>) -> Self {
        Self { name, properties }
    }

    /// Get the name of the scene object.
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// Get the properties of the scene object.
    pub fn properties(&self) -> &HashMap<Name, PropertyState> {
        &self.properties
    }

    /// Get the mutable properties of the scene object.
    pub fn properties_mut(&mut self) -> &mut HashMap<Name, PropertyState> {
        &mut self.properties
    }

    /// Get the property state for the given property name.
    pub fn property(&self, name: &Name) -> Option<&PropertyState> {
        self.properties.get(name)
    }
}

/// A default scene object with default properties and name.
///
/// Defined Properties:
///     position: vec3
///     orientation: vec3
///     velocity: vec3
///
/// The default scene object is automagically added to the scene when the engine is initialized.
/// this should not be used.
///
impl Default for SceneObject {
    fn default() -> Self {
        Self::new(
            "default".into(),
            HashMap::from([
                (name!("position"), PropertyState::unbound(Value::vec3())),
                (name!("orientation"), PropertyState::unbound(Value::vec3())),
                (name!("velocity"), PropertyState::unbound(Value::vec3())),
            ]),
        )
    }
}
