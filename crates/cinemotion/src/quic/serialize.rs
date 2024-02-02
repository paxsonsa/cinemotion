use std::collections::HashMap;

use bytes::{Buf, Bytes};

use super::stream::*;
use crate::commands::*;
use crate::data::*;
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

impl TryFrom<Frame> for Command {
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
                    CommandKind::Init => Ok(deserialize_init(payload)?.into()),
                }
            }
            _ => Ok(Command::Invalid),
        }
    }
}

fn deserialize_init(mut payload: Bytes) -> Result<Init, DeserializeError> {
    let name_len = payload.get_u16();
    let name = payload.split_to(name_len as usize).to_vec();
    let name = String::from_utf8(name).map_err(|_| DeserializeError::String)?;

    let num_properties = payload.get_u16();
    let mut properties: HashMap<Name, Property> = HashMap::with_capacity(num_properties as usize);
    println!("num: {}", num_properties);
    println!("{:?}", payload);
    for _ in 0..num_properties {
        let name_len = payload.get_u16();
        let name = payload.split_to(name_len as usize);
        println!("bytes: {:?}", name);
        let name = String::from_utf8(name.into()).map_err(|_| DeserializeError::String)?;
        println!("name: {:?}", name);
        let value = deserilize_value(&mut payload)?;
        let property = Property::with_default_value(name.into(), value);
        properties.insert(property.name.clone(), property);
    }

    Ok(Init {
        peer: Controller {
            name: name.into(),
            properties,
        },
    })
}

fn deserilize_value(payload: &mut Bytes) -> Result<Value, DeserializeError> {
    match payload.get_u8() {
        // Float
        1 => Ok(payload.get_f64().into()),
        // Vec3
        2 => Ok(Vec3 {
            x: payload.get_f64(),
            y: payload.get_f64(),
            z: payload.get_f64(),
        }
        .into()),
        // Vec4
        3 => Ok(Vec4 {
            x: payload.get_f64(),
            y: payload.get_f64(),
            z: payload.get_f64(),
            w: payload.get_f64(),
        }
        .into()),
        // Matrix44
        4 => Ok(Matrix44 {
            row0: Vec4 {
                x: payload.get_f64(),
                y: payload.get_f64(),
                z: payload.get_f64(),
                w: payload.get_f64(),
            },
            row1: Vec4 {
                x: payload.get_f64(),
                y: payload.get_f64(),
                z: payload.get_f64(),
                w: payload.get_f64(),
            },
            row2: Vec4 {
                x: payload.get_f64(),
                y: payload.get_f64(),
                z: payload.get_f64(),
                w: payload.get_f64(),
            },
            row3: Vec4 {
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
