use cinemotion::{commands, data, events, name, Event};
use cinemotion_proto as proto;
use proto::property_link;
use std::collections::HashMap;

#[test]
fn test_connection_opened_event_conversions() {
    let event = Event {
        target: None,
        body: events::ConnectionOpenedEvent().into(),
    };
    let proto_event: proto::Event = event.clone().into();
    matches!(
        proto_event.payload,
        Some(proto::event::Payload::ConnectionOpened(_))
    );
}

#[test]
fn test_init_message_conversion() {
    let message = proto::Command {
        payload: Some(proto::command::Payload::Init(proto::InitCommand {
            controller: Some(proto::ControllerDef {
                name: "test_controller".to_string(),
                properties: HashMap::from([(
                    "position".to_string(),
                    proto::PropertyValue {
                        value: Some(proto::property_value::Value::Vec3Value(proto::Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        })),
                    },
                )]),
            }),
        })),
    };
    let command = commands::Command::from_protobuf(message).expect("should pass");
    let commands::Command::Controller(commands::ControllerCommand::Init(init)) = command else {
        panic!("should be init command");
    };

    assert_eq!(init.peer.name, name!("test_controller"));
    let property = init
        .peer
        .properties
        .get(&name!("position"))
        .expect("property doesnt exist");
    matches!(property.default_value, data::Value::Vec3(data::Vec3 { .. }));
}

#[test]
fn test_change_mode_conversion() {
    let message = proto::Command {
        payload: Some(proto::command::Payload::ChangeMode(proto::ChangeMode {
            mode: proto::change_mode::Mode::Live.into(),
        })),
    };
    let command = commands::Command::from_protobuf(message).expect("should pass");
    let commands::Command::Controller(commands::ControllerCommand::ChangeMode(change_mode)) =
        command
    else {
        panic!("should be change mode command");
    };

    matches!(change_mode.0, data::Mode::Live);
}

#[test]
fn test_add_scene_object_conversion() {
    let message = proto::Command {
        payload: Some(proto::command::Payload::AddSceneObject(
            proto::AddSceneObject {
                object: Some(proto::SceneObject {
                    name: "test_object".to_string(),
                    properties: HashMap::from([
                        (
                            "position".to_string(),
                            proto::PropertyLink {
                                bind_state: proto::property_link::BindState::Bound.into(),
                                value: Some(proto::PropertyValue {
                                    value: Some(proto::property_value::Value::Vec3Value(
                                        proto::Vec3 {
                                            x: 0.0,
                                            y: 0.0,
                                            z: 0.0,
                                        },
                                    )),
                                }),
                                namespace: "test_namespace".to_string(),
                                property: "test_property".to_string(),
                            },
                        ),
                        (
                            "orientation".to_string(),
                            proto::PropertyLink {
                                bind_state: proto::property_link::BindState::Unbound.into(),
                                value: Some(proto::PropertyValue {
                                    value: Some(proto::property_value::Value::Vec3Value(
                                        proto::Vec3 {
                                            x: 0.0,
                                            y: 0.0,
                                            z: 0.0,
                                        },
                                    )),
                                }),
                                namespace: "".to_string(),
                                property: "".to_string(),
                            },
                        ),
                    ]),
                }),
            },
        )),
    };
    let command = commands::Command::from_protobuf(message).expect("should pass");
    let commands::Command::Controller(commands::ControllerCommand::AddSceneObject(add_object)) =
        command
    else {
        panic!("should be add scene object command");
    };

    assert_eq!(add_object.0.name(), &name!("test_object"));
    let property = add_object
        .0
        .property(&name!("position"))
        .expect("should have position property");

    matches!(property, data::PropertyLink::Bound { .. });

    let property = add_object
        .0
        .property(&name!("orientation"))
        .expect("should have orientation property");
    matches!(property, data::PropertyLink::Unbound { .. });
}
