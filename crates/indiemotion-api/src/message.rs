use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[cfg(test)]
#[path = "./message_test.rs"]
mod message_test;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type", content = "payload")]
pub enum Message {
    Command(crate::Command),
    State(crate::GlobalState),
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

pub struct Encoding<P> {
    _p: PhantomData<P>,
}

impl Encoding<JSONProtocol> {
    pub fn encode(message: &Message) -> Result<String> {
        serde_json::to_string(message).map_err(|err| Error::MessageEncoding(err.to_string()))
    }

    pub fn decode(message: &str) -> Result<Message> {
        serde_json::from_str(message).map_err(|err| Error::MessageEncoding(err.to_string()))
    }
}
