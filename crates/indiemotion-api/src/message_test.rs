use rstest::rstest;
use serde_json::to_string;
use super::*;
use crate::Echo;

#[rstest]
fn test_single_message_serde() {
    let message: ApiVersion = Message {
        header: Header {
            source: "test".to_string(),
            destination: None,
            source_time_ms: 1000,
        },
        payload: Payload::Single(
            Object::Echo(
                Echo {
                    message: "hello".to_string()
                }
            )
        )
    }.into();

    let json = serde_json::to_string(&message).expect("message should serialize");
    let de: ApiVersion = serde_json::from_str(&json).expect("message should deserialize");

    assert_eq!(message, de);
}

#[rstest]
fn test_multi_message_serde() {
    let message: ApiVersion = Message {
        header: Header {
            source: "test".to_string(),
            destination: None,
            source_time_ms: 1000,
        },
        payload: Payload::Multi(vec![
            Object::Echo(
                Echo {
                    message: "hello".to_string()
                }
            ),
            Object::Echo(
                Echo {
                    message: "world".to_string()
                }
            ),
        ])
    }.into();

    let json = serde_json::to_string(&message).expect("message should serialize");
    println!("{}", json);
    let de: ApiVersion = serde_json::from_str(&json).expect("message should deserialize");
    assert_eq!(message, de);
}