use super::State;
use crate::commands::{Event, Request};

pub trait Observer: Send + Sync {
    fn on_state_change(&mut self, new_state: &State);
    fn on_event(&mut self, event: &Event);
    fn on_request(&mut self, request: &Request);
}
