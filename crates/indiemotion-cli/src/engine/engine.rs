use api::models::PropertyState;
use api::Name;
use derive_more::Constructor;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::api;
use crate::Result;

#[derive(Constructor, Default)]
pub struct Engine {
    /// Map of client id to controller.
    controllers: HashMap<u32, Arc<api::models::ControllerState>>,

    // Map of controller name to client id.
    controller_client: HashMap<Name, u32>,

    /// The motion capture scene.
    scene: Arc<api::models::Scene>,

    /// The current motion mode.
    motion_mode: api::models::Mode,
}

impl Engine {
    pub async fn apply(&mut self, client_id: u32, command: api::Command) -> Result<()> {
        match command {
            api::Command::Empty => {}

            api::Command::SceneObject(object) => {
                (*Arc::make_mut(&mut self.scene)).add_object(object).await?;
            }
            api::Command::Controller(controller_def) => {
                if self.motion_mode.is_live() {
                    return Err(api::Error::BadMessage(
                        "cannot redefine controllers in live/recording mode".to_string(),
                    )
                    .into());
                }

                if let Some(existing_id) = self.controller_client.get(controller_def.name()) {
                    if existing_id != &client_id {
                        return Err(api::Error::BadMessage(format!(
                            "controller name already exists: '{}'",
                            controller_def.name()
                        ))
                        .into());
                    }
                }
                self.controller_client
                    .insert(controller_def.name().clone(), client_id);

                match self.controllers.get_mut(&client_id) {
                    Some(existing) => {
                        (*Arc::make_mut(existing)).redefine(controller_def);
                    }
                    None => {
                        self.controllers.insert(
                            client_id,
                            api::models::ControllerState::from(controller_def).into(),
                        );
                    }
                }
            }

            api::Command::Mode(mode) => {
                if mode.is_idle() && self.motion_mode.is_live() {
                    // Reset all controllers to their default values.
                    for controller in self.controllers.values_mut() {
                        (*Arc::make_mut(controller)).reset();
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
                    (*Arc::make_mut(controller))
                        .value_mut(&property.name)
                        .and_then(|v| v.update(&property.value).ok());
                }
            }
        }
        Ok(())
    }

    pub async fn tick(&mut self) -> Result<api::state::GlobalState> {
        // Update all object properties from their bindings.
        (*Arc::make_mut(&mut self.scene))
            .objects_mut()
            .iter_mut()
            .for_each(|(_, ref mut obj)| {
                obj.properties_mut()
                    .iter_mut()
                    .for_each(|(name, ref mut prop)| {

                        let PropertyState::Bound { value, binding } = prop else {
                            return;
                        };

                        let Some(controller) = self.controller_client.get(&binding.namespace).and_then(|i| self.controllers.get_mut(i)) else {
                            return;
                        };

                        let Some(ref_value) = controller.value(&binding.property) else {
                            return;
                        };

                        if let Err(err) = value.update(ref_value) {
                            tracing::error!(
                                "error updating property {}: {}",
                                name,
                                err
                            );
                        }
                    });
            });

        let state = api::GlobalState {
            controllers: self
                .controllers
                .values()
                .map(|p| (p.name().clone(), p.clone()))
                .collect(),
            scene: self.scene.clone(),
            mode: self.motion_mode.clone(),
        };

        Ok(state)
    }
}
