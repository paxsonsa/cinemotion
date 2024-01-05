use cinemotion::data::PropertyLink;
use paste::paste;
use std::collections::HashMap;

use cinemotion::{commands, data, name, scene, Message, State};

mod common;
use common::*;

harness!(
    motion_sampling,
    {
        let mut state = State::default();
        let mut controllers = HashMap::new();

        controllers.insert(
            name!("test"),
            data::Controller {
                uid: 1,
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
                }
            ),
        ]
    }
);
