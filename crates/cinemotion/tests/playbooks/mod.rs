/// End to end testing tools for Cinemotion.
use cinemotion::*;

pub struct Playbook {
    tasks: Vec<Task>,
}

pub struct Task {
    pub name: String,
    pub request: Request,
    pub expected_events: Vec<Event>,
    pub state: Option<String>,
}
