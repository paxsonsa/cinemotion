use cinemotion_proto::proto;

use super::*;
use crate::data::{motion::Mode, Sample};

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeMode(pub Mode);

impl From<ChangeMode> for Payload {
    fn from(value: ChangeMode) -> Self {
        Self::Client(ClientCommand::ChangeMode(value))
    }
}

impl From<proto::ChangeMode> for ChangeMode {
    fn from(value: proto::ChangeMode) -> Self {
        Self(value.mode().into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleMotion(pub Sample);

impl From<SampleMotion> for Payload {
    fn from(value: SampleMotion) -> Self {
        Self::Client(ClientCommand::SampleMotion(value))
    }
}

impl From<Sample> for SampleMotion {
    fn from(value: Sample) -> Self {
        Self(value)
    }
}

impl From<proto::SendSample> for SampleMotion {
    fn from(value: proto::SendSample) -> Self {
        let Some(sample) = value.sample else {
            return SampleMotion(Sample::empty());
        };

        Self(sample.into())
    }
}
