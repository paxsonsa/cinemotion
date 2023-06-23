use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[cfg(test)]
#[path = "./message_test.rs"]
mod message_test;

/// A message is the unit of communication between the client and server.
///
/// Messages can be one of three types:
/// - Command: A command to the server to perform some action.
/// - State: A state update from the server to the client.
/// - Error: An error message from the server to the client (or vice versa).
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "payload")]
pub enum Message {
    /// A command to the server to perform some action.
    Command(crate::Command),
    /// A state update from the server to the client.
    State(crate::GlobalState),
    /// An error message from the server to the client (or vice versa).
    Error(crate::Error),
}

impl TryFrom<String> for Message {
    type Error = Error;

    fn try_from(msg: String) -> Result<Self> {
        match Encoding::<JSONProtocol>::decode(&msg) {
            Ok(msg) => Ok(msg),
            Err(err) => Err(Error::MessageEncoding(err.to_string())),
        }
    }
}

pub struct JSONProtocol {}

/// A Generic encoding/decoding protocol.
///
/// Supported protocols:
/// - JSONProtocol
///
pub struct Encoding<P> {
    _p: PhantomData<P>,
}

/// An implementation of the JSONProtocol encoding/decoding protocol.
impl Encoding<JSONProtocol> {
    /// Encode the given message into a string.
    ///
    /// ```
    /// use indiemotion_api::{message::{Encoding, JSONProtocol}, Error, Message};
    /// let command = Message::Error(Error::ControllerError("an error occured".into()));
    /// println!(
    ///     "{}",
    ///     Encoding::<JSONProtocol>::encode(&command).expect("message should serialize")
    /// );
    /// ```
    pub fn encode(message: &Message) -> Result<String> {
        serde_json::to_string(message).map_err(|err| Error::MessageEncoding(err.to_string()))
    }

    /// Decode the given json string into a message instance.
    ///
    /// ```
    /// use indiemotion_api::{message::{Encoding, JSONProtocol}, Error, Message};
    /// let data = r#"
    /// {
    ///     "type": "error",
    ///     "payload": {
    ///       "error_type": "ControllerError",
    ///       "message": "an error occured"
    ///     }
    ///   }
    /// "#;
    /// let message: Message = serde_json::from_str(data).expect("message should deserialize");
    /// assert!(matches!(message, Message::Error(Error::ControllerError(_))));
    /// ```
    ///
    ///
    pub fn decode(message: &str) -> Result<Message> {
        serde_json::from_str(message).map_err(|err| Error::MessageEncoding(err.to_string()))
    }
}
