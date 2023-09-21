use super::*;
use crate::{models::*, GlobalState};
use crate::{name, Command};
use serde_json;
use std::collections::HashMap;

#[test]
fn test_message_command_empty_serde() {
    let command = Message::Command(Command::Empty);
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );

    let data = r#"{
        "type": "command",
        "payload": {
            "command": "empty"
        }
    }"#;

    let message = Encoding::<JSONProtocol>::decode(data).expect("message should serialize");
    assert!(matches!(message, Message::Command(Command::Empty)));
}

#[test]
fn test_message_command_controlledef_serde() {
    let command = Message::Command(Command::Controller(ControllerDef::new(
        "controllerA".into(),
        vec![PropertyDef::new(name!("position"), (0.0, 0.0, 0.0).into())],
    )));
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );

    let data = r#"
    {
        "type": "command",
        "payload": {
          "command": "controller",
          "name": "controllerA",
          "properties": [
            {
              "name": "position",
              "default_value": {
                "x": 0.0,
                "y": 0.0,
                "z": 0.0
              }
            }
          ]
        }
      }
    "#;
    let message: Message = serde_json::from_str(data).expect("message should deserialize");
    assert!(matches!(message, Message::Command(Command::Controller(_))));
}

#[test]
fn test_message_command_scene_object_serde() {
    let command = Message::Command(Command::SceneObject(SceneObject::new(
        "objectA".into(),
        HashMap::from([
            (
                name!("position"),
                PropertyState::bind(name!("controllerA"), name!("position"), Value::vec3()),
            ),
            (name!("rotate"), PropertyState::unbound(Value::vec3())),
            (name!("scale"), PropertyState::unbound(Value::vec3())),
        ]),
    )));
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );

    let data = r#"
    {
        "type":"command",
        "payload":{
           "command":"sceneobject",
           "name":"objectA",
           "properties":{
              "scale":{
                 "value":{
                    "x":0.0,
                    "y":0.0,
                    "z":0.0
                 }
              },
              "position":{
                 "value":{
                    "x":0.0,
                    "y":0.0,
                    "z":0.0
                 },
                 "binding":{
                    "namespace":"controllerA",
                    "property":"position"
                 }
              },
              "rotate":{
                 "value":{
                    "x":0.0,
                    "y":0.0,
                    "z":0.0
                 }
              }
           }
        }
     }
    "#;

    let message: Message = serde_json::from_str(data).expect("message should deserialize");
    assert!(matches!(message, Message::Command(Command::SceneObject(_))));
}

#[test]
fn test_message_command_sample_serde() {
    let command = Message::Command(Command::Sample(Sample::new(HashMap::from([(
        name!("position"),
        (1.0, 1.0, 1.0).into(),
    )]))));
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );
    let data = r#"
    {
        "type": "command",
        "payload": {
          "command": "sample",
          "properties": {
            "position": {
              "x": 1,
              "y": 1,
              "z": 1
            }
          }
        }
      }
    "#;
    let message: Message = serde_json::from_str(data).expect("message should deserialize");
    assert!(matches!(message, Message::Command(Command::Sample(_))));
}

#[test]
fn test_message_command_mode_serde() {
    let command = Message::Command(Command::Mode(Mode::Live));
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );
    let data = r#"
    {
        "type": "command",
        "payload": {
          "command": "mode",
          "mode": "live"
        }
      }
    "#;
    let message: Message = serde_json::from_str(data).expect("message should deserialize");
    assert!(matches!(message, Message::Command(Command::Mode(_))));
}

#[test]
fn test_message_state_serde() {
    let command = Message::State(GlobalState::default());
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );
    let data = r#"
    {
        "type": "state",
        "payload": {
          "controllers": {},
          "scene": {
            "name": "default",
            "objects": {
              "default": {
                "name": "default",
                "properties": {
                  "position": {
                    "value": {
                      "x": 0,
                      "y": 0,
                      "z": 0
                    }
                  },
                  "orientation": {
                    "value": {
                      "x": 0,
                      "y": 0,
                      "z": 0
                    }
                  },
                  "velocity": {
                    "value": {
                      "x": 0,
                      "y": 0,
                      "z": 0
                    }
                  }
                }
              }
            }
          },
          "mode": {
            "mode": "idle"
          }
        }
      }
    "#;
    let message: Message = serde_json::from_str(data).expect("message should deserialize");
    assert!(matches!(message, Message::State(_)));
}

#[test]
fn test_message_command_error_serde() {
    let command = Message::Error(Error::ControllerError("an error occured".into()));
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );
    let data = r#"
    {
        "type": "error",
        "payload": {
          "error_type": "ControllerError",
          "message": "an error occured"
        }
      }
    "#;
    let message: Message = serde_json::from_str(data).expect("message should deserialize");
    assert!(matches!(message, Message::Error(Error::ControllerError(_))));
}

#[test]
fn test_message_echo_message_serde() {
    let command = Message::Echo("hello, world".into());
    println!(
        "{}",
        Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    );

    let data = r#"{
        "type": "echo",
        "payload": "hello, world"
    }"#;

    let message = Encoding::<JSONProtocol>::decode(data).expect("message should serialize");
    assert!(matches!(message, Message::Echo(s) if s == "hello, world".to_string()));
}
