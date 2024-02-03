use crate::messages;
use bytes::{Buf, Bytes};
use thiserror::Error;

#[cfg(test)]
#[path = "stream_test.rs"]
mod stream_test;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum FrameError {
    #[error("invalid frame: {0}")]
    InvalidFrame(String),
}

pub enum FrameType {
    Command,
    Error,
    Invalid(u8),
}

impl From<u8> for FrameType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Command,
            1 => Self::Error,
            _ => Self::Invalid(value),
        }
    }
}

pub struct Frame {
    pub api_version: u8,
    pub kind: u8,
    pub payload_length: u32,
    pub payload: Bytes,
}

impl Frame {
    pub async fn from_stream<T>(stream: &mut T) -> Result<Self, FrameError>
    where
        T: tokio::io::AsyncReadExt + Send + Unpin,
    {
        let mut buf = bytes::BytesMut::with_capacity(1024);
        // Temporarily set the length of the buffer to 8 bytes for the header read
        let original_len = buf.len();
        unsafe {
            buf.set_len(8);
        }
        stream
            .read_exact(&mut buf)
            .await
            .map_err(|err| FrameError::InvalidFrame(err.to_string()))?;

        let api_version = buf.get_u8();
        let frame_type = buf.get_u8();
        let payload_length = buf.get_u32();
        let _ = buf.get_u16(); // padding
        unsafe {
            buf.set_len(original_len);
        }
        buf.resize(payload_length as usize, 0);

        stream
            .read_exact(&mut buf)
            .await
            .map_err(|err| FrameError::InvalidFrame(err.to_string()))?;

        Ok(Frame {
            api_version,
            kind: frame_type,
            payload_length,
            payload: buf.freeze(),
        })
    }

    pub fn frame_type(&self) -> FrameType {
        self.kind.into()
    }
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum RecvError {
    #[error("frame error occured while reading bytes: {0}")]
    FrameError(String),

    #[error("invalid frame type was received: {0}")]
    InvalidFrameType(u8),

    #[error("some invoked functionality is not implemented yet")]
    NotImplemented,
}

pub async fn recv_command<T>(stream: &mut T) -> Result<messages::Payload, RecvError>
where
    T: tokio::io::AsyncReadExt + Send + Sync + Unpin,
{
    let frame = Frame::from_stream(stream)
        .await
        .map_err(|err| RecvError::FrameError(err.to_string()))?;

    match frame.frame_type() {
        FrameType::Command => {
            todo!();
            // let command: messages::Command = frame.into();
            // Ok(command)
        }
        FrameType::Error => Err(RecvError::NotImplemented),
        FrameType::Invalid(kind_id) => Err(RecvError::InvalidFrameType(kind_id)),
    }
}
