use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("channel closed: {0}")]
    ChannelClosed(&'static str),

    #[error("failed to complete signaling due to error: {0}")]
    SignalingFailed(String),

    #[error("webrtc error occurred: {0}")]
    WebRTCError(#[from] webrtc::Error),

    #[error("session failed: {0}")]
    SessionFailed(&'static str),

    #[error("bad session descriptor: {0}")]
    BadSessionDescriptor(String),
}
