use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("channel closed: {0}")]
    ChannelClosed(&'static str),

    #[error("failed to complete signaling due to error: {0}")]
    SignalingFailed(String),

    #[error("webrtc error occurred: {0}")]
    WebRTCError(String),

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

    #[error("invalid mode: {0}")]
    InvalidMode(String),
}

impl From<webrtc::Error> for Error {
    fn from(error: webrtc::Error) -> Self {
        Self::WebRTCError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, self::Error>;
