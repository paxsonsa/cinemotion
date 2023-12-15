#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;
use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/cinemotion.rs"));
}
pub use proto::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("decoding error occurred: {0}")]
    DecodingError(#[from] prost::DecodeError),

    #[error("encoding error occurred: {0}")]
    EncodingError(#[from] prost::EncodeError),
}

impl TryInto<bytes::Bytes> for Event {
    type Error = self::Error;
    fn try_into(self) -> Result<bytes::Bytes, Self::Error> {
        let mut buf = bytes::BytesMut::new();
        self.encode(&mut buf)?;
        Ok(buf.freeze())
    }
}

impl TryFrom<bytes::Bytes> for Command {
    type Error = self::Error;

    fn try_from(value: bytes::Bytes) -> Result<Self, Self::Error> {
        match prost::Message::decode(value) {
            Ok(msg) => Ok(msg),
            Err(err) => Err(err.into()),
        }
    }
}

// This is a convenience macro to implement the From trait for our
// generated protobuf types.
macro_rules! impl_from_payload {
    ($type:ident) => {
        impl From<$type> for command::Payload {
            fn from(request: $type) -> Self {
                command::Payload::$type(request)
            }
        }
    };
}

impl_from_payload!(Echo);
