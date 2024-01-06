use cinemotion_proto::proto;

use super::*;
use crate::data::{motion::Mode, Sample};

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeMode(pub Mode);

impl From<ChangeMode> for Command {
    fn from(value: ChangeMode) -> Self {
        Self::Controller(ControllerCommand::ChangeMode(value))
    }
}

impl From<proto::ChangeMode> for ChangeMode {
    fn from(value: proto::ChangeMode) -> Self {
        Self(value.mode().into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SampleMotion(pub Sample);

impl From<SampleMotion> for Command {
    fn from(value: SampleMotion) -> Self {
        Self::Controller(ControllerCommand::SampleMotion(value))
    }
}

impl From<Sample> for SampleMotion {
    fn from(value: Sample) -> Self {
        Self(value)
    }
}
