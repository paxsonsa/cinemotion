use cinemotion::data::PropertyLink;
use paste::paste;
use std::collections::HashMap;

use cinemotion::{commands, data, name, scene, Message, State};

mod common;
use common::*;

harness!(
    basic_motion_sampling,
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

        let object = scene::SceneObject::new(
            name!("object1"),
            HashMap::from([(
                name!("position"),
                PropertyLink::bind(
                    name!("test"),
                    name!("position"),
                    data::Value::Vec3((0.0, 0.0, 0.0).into()),
                ),
            )]),
        );
        state
            .scene
            .add_object(object)
            .await
            .expect("object should be added");
        state
    },
    {
        vec![
            message!(
                "update the motion mode to be live",
                Message {
                    source_id: 1,
                    command: commands::ChangeMode(data::Mode::Live).into(),
                }
            ),
            state!("verify that the mode is live", |state: &mut State| {
                state.mode = data::Mode::Live;
            }),
            message!(
                "send a motion sample",
                Message {
                    source_id: 1,
                    command: commands::SampleMotion(data::Sample::new(HashMap::from([(
                        name!("position"),
                        data::Value::Vec3((0.0, 1.0, 0.0).into())
                    ),])))
                    .into(),
                }
            ),
            state!(
                "verify that sample is applied to the object",
                |state: &mut State| {
                    state
                        .scene
                        .objects_mut()
                        .get_mut(&name!("object1"))
                        .expect("object1 to exist")
                        .properties_mut()
                        .insert(
                            name!("position"),
                            data::PropertyLink::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 1.0, 0.0).into()),
                            ),
                        );
                    state
                        .controllers
                        .get_mut(&name!("test"))
                        .expect("controller should exist")
                        .properties
                        .get_mut(&name!("position"))
                        .expect("property must exist")
                        .value = data::Value::Vec3((0.0, 1.0, 0.0).into());
                }
            ),
        ]
    }
);

harness!(
    motion_sampling_disabled_when_idle,
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

        let object = scene::SceneObject::new(
            name!("object1"),
            HashMap::from([(
                name!("position"),
                PropertyLink::bind(
                    name!("test"),
                    name!("position"),
                    data::Value::Vec3((0.0, 0.0, 0.0).into()),
                ),
            )]),
        );
        state
            .scene
            .add_object(object)
            .await
            .expect("object should be added");
        state
    },
    {
        vec![
            message!(
                "send a motion sample",
                Message {
                    source_id: 1,
                    command: commands::SampleMotion(data::Sample::new(HashMap::from([(
                        name!("position"),
                        data::Value::Vec3((0.0, 1.0, 0.0).into())
                    ),])))
                    .into(),
                }
            ),
            state!(
                "verify that sample is NOT applied to the object",
                |state: &mut State| {
                    state
                        .scene
                        .objects_mut()
                        .get_mut(&name!("object1"))
                        .expect("object1 to exist")
                        .properties_mut()
                        .insert(
                            name!("position"),
                            data::PropertyLink::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into()),
                            ),
                        );
                    state
                        .controllers
                        .get_mut(&name!("test"))
                        .expect("controller should exist")
                        .properties
                        .get_mut(&name!("position"))
                        .expect("property must exist")
                        .value = data::Value::Vec3((0.0, 0.0, 0.0).into());
                }
            ),
        ]
    }
);

harness!(
    motion_sampling_resets_when_going_back_to_idle,
    {
        let mut state = State {
            mode: data::Mode::Live,
            ..Default::default()
        };
        let mut controllers = HashMap::new();

        controllers.insert(
            name!("test"),
            data::Controller {
                name: name!("test"),
                properties: vec![data::Property {
                    name: name!("position"),
                    default_value: data::Value::Vec3((0.0, 0.0, 0.0).into()),
                    value: data::Value::Vec3((0.0, 10.0, 0.0).into()),
                }]
                .into_iter()
                .map(|p| (p.name.clone(), p))
                .collect(),
            },
        );

        state.controllers = controllers;

        let object = scene::SceneObject::new(
            name!("object1"),
            HashMap::from([(
                name!("position"),
                PropertyLink::bind(
                    name!("test"),
                    name!("position"),
                    data::Value::Vec3((0.0, 10.0, 0.0).into()),
                ),
            )]),
        );
        state
            .scene
            .add_object(object)
            .await
            .expect("object should be added");
        state
    },
    {
        vec![
            message!(
                "change mode to idle",
                Message {
                    source_id: 1,
                    command: commands::ChangeMode(data::Mode::Idle).into(),
                }
            ),
            state!(
                "verify that values are reset to defaults applied to the object",
                |state: &mut State| {
                    state
                        .scene
                        .objects_mut()
                        .get_mut(&name!("object1"))
                        .expect("object1 to exist")
                        .properties_mut()
                        .insert(
                            name!("position"),
                            data::PropertyLink::bind(
                                name!("test"),
                                name!("position"),
                                data::Value::Vec3((0.0, 0.0, 0.0).into()),
                            ),
                        );
                    state
                        .controllers
                        .get_mut(&name!("test"))
                        .expect("controller should exist")
                        .properties
                        .get_mut(&name!("position"))
                        .expect("property must exist")
                        .value = data::Value::Vec3((0.0, 0.0, 0.0).into());
                }
            ),
        ]
    }
);
