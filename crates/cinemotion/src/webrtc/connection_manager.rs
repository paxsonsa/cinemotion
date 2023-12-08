use crate::Result;

use super::SessionDescriptor;

pub struct ConnectionManager {}

impl ConnectionManager {
    pub fn new() -> Self {
        ConnectionManager {}
    }

    pub fn create_connection(
        &mut self,
        session_desc: SessionDescriptor,
    ) -> Result<SessionDescriptor> {
        Ok(SessionDescriptor {
            payload: "hello".to_string(),
        })
    }
}
