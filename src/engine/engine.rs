use std::collections::HashMap;

use api::models::ObjectProperty;
use derive_more::Constructor;

use crate::api;
use crate::Result;

#[derive(Constructor, Default)]
pub struct Engine {
    /// Map of client id to controller.
    controllers: HashMap<u32, api::models::Controller>,

    // Map of controller name to client id.
    controller_client: HashMap<String, u32>,

    /// The motion capture scene.
    scene: api::models::Scene,

    /// The current motion mode.
    motion_mode: api::models::Mode,
}

impl Engine {
    pub async fn apply(&mut self, client_id: u32, command: api::Command) -> Result<()> {
        match command {
            api::Command::Empty => {}

            api::Command::SceneObject(object) => {
                self.scene.add_object(object).await?;
            }
            api::Command::Controller(controller) => {
                if self.controllers.get_mut(&client_id).is_none() {
                    let properties: Vec<ObjectProperty> = controller
                        .properties()
                        .iter()
                        .map(|p| {
                            api::models::ObjectProperty::new(
                                p.name().to_string(),
                                p.default_value().clone(),
                                Some(api::models::PropertyBinding {
                                    namespace: controller.name().to_string(),
                                    property: p.name().to_string(),
                                }),
                            )
                        })
                        .collect();

                    let object = api::models::SceneObject::new(
                        controller.name().to_string().into(),
                        properties,
                    );
                    let _ = self.scene.add_object(object).await;
                }
                self.controller_client
                    .insert(controller.name().to_string(), client_id);
                self.controllers.insert(client_id, controller);
            }

            api::Command::Mode(mode) => {
                if mode.is_idle() && self.motion_mode.is_live() {
                    // Reset all controllers to their default values.
                    for controller in self.controllers.values_mut() {
                        for property in controller.properties_mut() {
                            property.reset();
                        }
                    }
                }
                self.motion_mode = mode;
            }

            api::Command::Sample(sample) => {
                if self.motion_mode.is_idle() {
                    return Ok(());
                }

                let Some(controller) = self.controllers.get_mut(&client_id) else {
                    return Err(api::Error::BadMessage("controller must be assigned before sending samples".to_string()).into());
                };

                for property in sample.properties() {
                    if let Some(prop) = controller.property_mut(&property.name) {
                        if let Err(err) = prop.value_mut().update(&property.value) {
                            tracing::error!("error sampling property {}: {}", property.name, err);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn tick(&mut self) -> Result<api::state::GlobalState> {
        // Update all object properties from their bindings.
        self.scene
            .objects_mut()
            .iter_mut()
            .for_each(|(_, ref mut obj)| {
                obj.properties_mut()
                    .iter_mut()
                    .for_each(|(_, ref mut prop)| {
                        let Some(binding) = prop.binding() else {
                            return;
                        };

                        let Some(controller) = self.controller_client.get(&binding.namespace).and_then(|i| self.controllers.get_mut(i)) else {
                            return;
                        };

                        let Some(controller_prop) = controller.property(&binding.property) else {
                            return;
                        };

                        if let Err(err) = prop.value_mut().update(controller_prop.value()) {
                            tracing::error!(
                                "error updating property {}: {}",
                                prop.name().to_string(),
                                err
                            );
                        }
                    });
            });

        let state = api::GlobalState {
            controllers: self
                .controllers
                .iter()
                .map(|x| (x.1.name().to_string(), x.1.clone()))
                .collect(),
            scene: self.scene.clone(),
            mode: self.motion_mode.clone(),
        };

        Ok(state)
    }
}
