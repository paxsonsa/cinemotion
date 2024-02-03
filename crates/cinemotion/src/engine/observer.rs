use crate::State;
use crate::{messages, Event};

pub trait Observer: Send + Sync {
    fn on_state_change(&mut self, new_state: &State);
    fn on_event(&mut self, event: &Event);
    fn on_message(&mut self, message: &messages::Message);
}
