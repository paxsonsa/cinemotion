use crate::{Error, Result};
use base64::prelude::{Engine, BASE64_STANDARD_NO_PAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WebRTCSessionDescriptor {
    pub payload: String,
}

impl WebRTCSessionDescriptor {
    /// create a new session descriptor with the raw SDP string
    /// the payload is base64 encoded.
    pub fn new(desc_raw: &str) -> Self {
        Self {
            payload: BASE64_STANDARD_NO_PAD.encode(desc_raw),
        }
    }

    /// Create a new decriptor with the given encoded payload.
    pub fn new_encode(payload: String) -> Self {
        Self { payload }
    }

    /// Return the decoded session desriptor payload.
    pub fn decode(&self) -> Result<String> {
        let payload = match BASE64_STANDARD_NO_PAD.decode(&self.payload) {
            Ok(s) => s,
            Err(err) => {
                return Err(Error::BadRTCDescriptor(format!(
                    "failed to decode base64 payload: {err}"
                )))
            }
        };
        let s = match String::from_utf8(payload) {
            Ok(s) => s,
            Err(err) => {
                return Err(Error::BadRTCDescriptor(format!(
                    "failed to decode utf8 string from decoded payload: {err}"
                )))
            }
        };
        Ok(s)
    }
}
