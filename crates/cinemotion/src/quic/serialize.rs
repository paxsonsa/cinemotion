use bytes::{Buf, Bytes};
use std::collections::HashMap;

use super::stream::*;
use crate::data;
use crate::messages;
use crate::Name;

#[cfg(test)]
#[path = "serialize_test.rs"]
mod serialize_test;

enum CommandKind {
    Init,
}

impl From<u8> for CommandKind {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Init,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DeserializeError {
    #[error("frame could not be deserialized")]
    BadFrame,

    #[error("failed to decode utf8 string")]
    String,

    #[error("value could not be deserialized")]
    Value,
}

impl TryFrom<Frame> for messages::Payload {
    type Error = DeserializeError;

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        if frame.api_version != 1 {
            return Err(DeserializeError::BadFrame);
        }

        let mut payload = frame.payload.clone();

        match frame.frame_type() {
            FrameType::Command => {
                // Read the command id from the payload
                let kind: CommandKind = payload.get_u8().into();
                match kind {
                    CommandKind::Init => Ok(Self::Client(messages::ClientCommand::Init(
                        messages::Init::try_from(&mut payload)?,
                    ))),
                }
            }
            _ => Ok(messages::Payload::Invalid),
        }
    }
}

/// Deserialize a `messages::Init` from a quic byte buffer.
impl TryFrom<&mut QuicBytes> for messages::Init {
    type Error = DeserializeError;

    fn try_from(payload: &mut QuicBytes) -> Result<Self, Self::Error> {
        let name_len = payload.get_u16();
        let name = payload.split_to(name_len as usize).to_vec();
        let name = String::from_utf8(name).map_err(|_| DeserializeError::String)?;
        let num_properties = payload.get_u16();
        let mut properties: HashMap<Name, data::Property> =
            HashMap::with_capacity(num_properties as usize);
        for _ in 0..num_properties {
            let name_len = payload.get_u16();
            let name = payload.split_to(name_len as usize);
            let name = String::from_utf8(name.into()).map_err(|_| DeserializeError::String)?;
            let value = data::Value::try_from(&mut *payload)?;
            let property = data::Property::with_default_value(name.into(), value);
            properties.insert(property.name.clone(), property);
        }
        Ok(Self {
            peer: data::Controller {
                name: name.into(),
                properties,
            },
        })
    }
}

// impl TryFrom<&mut QuicBytes> for data::Controller {
//     type Error = DeserializeError;
//     fn try_from(payload: &mut QuicBytes) -> Result<Self, Self::Error> {
//
//     }
// }

impl TryFrom<&mut QuicBytes> for data::Value {
    type Error = DeserializeError;
    fn try_from(payload: &mut QuicBytes) -> Result<Self, Self::Error> {
        match payload.get_u8() {
            // Float
            1 => Ok(payload.get_f64().into()),
            // Vec3
            2 => Ok(data::Vec3 {
                x: payload.get_f64(),
                y: payload.get_f64(),
                z: payload.get_f64(),
            }
            .into()),
            // Vec4
            3 => Ok(data::Vec4 {
                x: payload.get_f64(),
                y: payload.get_f64(),
                z: payload.get_f64(),
                w: payload.get_f64(),
            }
            .into()),
            // Matrix44
            4 => Ok(data::Matrix44 {
                row0: data::Vec4 {
                    x: payload.get_f64(),
                    y: payload.get_f64(),
                    z: payload.get_f64(),
                    w: payload.get_f64(),
                },
                row1: data::Vec4 {
                    x: payload.get_f64(),
                    y: payload.get_f64(),
                    z: payload.get_f64(),
                    w: payload.get_f64(),
                },
                row2: data::Vec4 {
                    x: payload.get_f64(),
                    y: payload.get_f64(),
                    z: payload.get_f64(),
                    w: payload.get_f64(),
                },
                row3: data::Vec4 {
                    x: payload.get_f64(),
                    y: payload.get_f64(),
                    z: payload.get_f64(),
                    w: payload.get_f64(),
                },
            }
            .into()),
            // Catch all
            _ => Err(DeserializeError::Value),
        }
    }
}
