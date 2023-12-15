use super::Command;
use super::Echo;
use prost::Message;

#[test]
fn test_echo_request_size() {
    let message = Command {
        payload: Some(
            Echo {
                message: "hello world".into(),
            }
            .into(),
        ),
    };
    let mut buf = Vec::new();
    message.encode(&mut buf).unwrap();
    assert!(buf.len() < 16 * 1024, "Serialized size exceeds 16kb");
}
