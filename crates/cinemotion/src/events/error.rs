use crate::Error;
use cinemotion_proto as proto;

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorEvent(pub Error);

impl From<Error> for ErrorEvent {
    fn from(error: Error) -> Self {
        Self(error)
    }
}

impl From<ErrorEvent> for proto::ErrorEvent {
    fn from(value: ErrorEvent) -> Self {
        let description = value.0.to_string();
        let error_type: proto::error_event::ErrorType = match value.0 {
            Error::InvalidSceneObject(_) => proto::error_event::ErrorType::InvalidSceneObject,
            _ => proto::error_event::ErrorType::Unknown,
        };
        proto::ErrorEvent {
            description,
            r#type: error_type.into(),
        }
    }
}
