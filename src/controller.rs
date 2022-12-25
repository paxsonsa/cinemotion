use crate::api;
use crate::Result;

pub enum ControllerUpdateResult {
    /// The message was handled successfully and no further action is required.
    Stop,

    /// The message was handled successfully and it should be forwarded to the 
    /// next controller in the chain.
    Next,

    /// The message was handled successfully, the contained response should be sent and 
    /// the message shoud be forwarded to the next controller in the chain.
    RespondNext(api::Message),

    /// The message was handled successfully, the contained response should be sent and 
    /// no further action is required.
    RespondStop(api::Message),
}

impl Default for ControllerUpdateResult {
    fn default() -> Self {
        ControllerUpdateResult::Next
    }
}

#[crate::async_trait]
pub trait Controller {
    /// Returns the name of the controller.
    fn name() -> &'static str;

    /// Update the controller with a new message.
    async fn update(&self, time: api::TimeSpec, message: api::Message) -> Result<ControllerUpdateResult>;
}
