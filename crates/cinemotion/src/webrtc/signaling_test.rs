use core::panic;
use std::time::Duration;

use super::*;

#[tokio::test]
async fn test_signaling_create() {
    let (sender, mut receiver) = crate::commands::command_channel();
    let relay = SignalingRelay::new(sender);

    let session_desc = SessionDescriptor {
        payload: "hello".to_string(),
    };

    let handle = tokio::spawn(async move {
        match receiver.recv().await {
            Some(command) => {
                if let crate::commands::Command::CreateSession(new_session) = command {
                    let desc = SessionDescriptor {
                        payload: "world!".to_string(),
                    };
                    if let Err(err) = new_session.sender.send(Ok(desc)) {
                        panic!("failed to send: {err:?}")
                    }
                } else {
                    panic!("incorrect type")
                }
            }
            None => panic!("got none."),
        }
    });

    let result = tokio::time::timeout(Duration::from_secs(1), relay.create(session_desc))
        .await
        .unwrap();
    assert!(result.is_ok());
    let _ = tokio::time::timeout(Duration::from_secs(1), handle)
        .await
        .unwrap();
}
