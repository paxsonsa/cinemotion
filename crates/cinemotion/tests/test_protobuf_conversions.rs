use cinemotion::{commands, data, events, name, Event};
use cinemotion_proto as proto;
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
