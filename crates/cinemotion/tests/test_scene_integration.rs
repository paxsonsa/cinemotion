use cinemotion::data::PropertyLink;
use paste::paste;
use std::collections::HashMap;

use cinemotion::{commands, data, name, scene, Error, Event, EventBody, Message, State};

mod common;
use common::*;

harness!(
    scene_object_commands,
    {
        let mut state = State::default();
        let mut controllers = HashMap::new();
        controllers.insert(
            name!("test"),
            data::Controller {
                name: name!("test"),
                properties: vec![data::Property::with_default_value(
                    name!("position"),
                    data::Value::Vec3((0.0, 0.0, 0.0).into()),
                )]
                .into_iter()
                .map(|p| (p.name.clone(), p))
                .collect(),
            },
        );
        state.controllers = controllers;
        state
    },
    {
        vec![
            message!(
                "attempt to update a scene object that does not exist",
                Message {
                    source_id: 1,
                    command: commands::UpdateSceneObject(scene::SceneObject::new(
                        name!("doesnotexist"),
                        HashMap::from([(
                            name!("position"),
                            data::PropertyLink::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into())
                            ),
                        )])
                    ))
                    .into(),
                }
            ),
            event!("expect an error event to be emitted", |event: &Event| {
                match event.target {
                    Some(1) => match &event.body {
                        EventBody::Error(event) => {
                            matches!(event.0, Error::InvalidSceneObject(_))
                        }
                        _ => false,
                    },

                    _ => false,
                }
            }),
            message!(
                "add a new scene object.",
                Message {
                    source_id: 1,
                    command: commands::AddSceneObject(scene::SceneObject::new(
                        name!("object1"),
                        HashMap::from([(
                            name!("position"),
                            PropertyLink::unbound(data::Vec3::from((0.0, 0.0, 0.0)).into()),
                        )])
                    ))
                    .into(),
                }
            ),
            state!(
                "verify that the scene object is in the state",
                |state: &mut State| {
                    state.scene.objects_mut().insert(
                        name!("object1"),
                        scene::SceneObject::new(
                            name!("object1"),
                            HashMap::from([(
                                name!("position"),
                                PropertyLink::unbound(data::Vec3::from((0.0, 0.0, 0.0)).into()),
                            )]),
                        ),
                    );
                }
            ),
            message!(
                "try to add a existing scene object.",
                Message {
                    source_id: 1,
                    command: commands::AddSceneObject(scene::SceneObject::new(
                        name!("object1"),
                        HashMap::from([(
                            name!("position"),
                            PropertyLink::unbound(data::Vec3::from((0.0, 0.0, 0.0)).into()),
                        )])
                    ))
                    .into(),
                }
            ),
            event!("expect an error event to be emitted", |event: &Event| {
                match event.target {
                    Some(1) => match &event.body {
                        EventBody::Error(event) => {
                            matches!(event.0, Error::InvalidSceneObject(_))
                        }
                        _ => false,
                    },

                    _ => false,
                }
            }),
            message!(
                "update the root scene object to map controller property to object",
                Message {
                    source_id: 1,
                    command: commands::UpdateSceneObject(scene::SceneObject::new(
                        name!("default"),
                        HashMap::from([(
                            name!("position"),
                            data::PropertyLink::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into())
                            ),
                        )])
                    ))
                    .into(),
                }
            ),
            state!(
                "verify that the scene objects property is bound to the controller",
                |state: &mut State| {
                    state
                        .scene
                        .objects_mut()
                        .get_mut(&name!("default"))
                        .expect("expected default object")
                        .properties_mut()
                        .insert(
                            name!("position"),
                            data::PropertyLink::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into()),
                            ),
                        );
                }
            ),
            message!(
                "delete object in the scene",
                Message {
                    source_id: 1,
                    command: commands::DeleteSceneObject(name!("object1"),).into(),
                }
            ),
            state!("check that the object was deleted", |state: &mut State| {
                let _ = state.scene.objects_mut().remove(&name!("object1"));
            }),
        ]
    }
);
