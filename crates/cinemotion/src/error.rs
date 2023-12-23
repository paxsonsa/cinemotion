use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("channel closed: {0}")]
    ChannelClosed(&'static str),

    #[error("failed to complete signaling due to error: {0}")]
    SignalingFailed(String),

    #[error("webrtc error occurred: {0}")]
    WebRTCError(#[from] webrtc::Error),

    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error("bad webrtc descriptor: {0}")]
    BadRTCDescriptor(String),

    #[error("engine failed: {0}")]
    EngineFailed(String),

    #[error("bad command error: {0}")]
    BadCommand(String),

    #[error("invalid scene object: {0}")]
    InvalidSceneObject(String),

    #[error("invalid property value: {0}")]
    InvalidValue(String),
}

pub type Result<T> = std::result::Result<T, self::Error>;
