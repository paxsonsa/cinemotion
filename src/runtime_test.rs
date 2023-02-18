use rstest::rstest;

use super::*;

#[derive(Debug)]
struct DummyClientRelay {}

impl api::ClientRelay for DummyClientRelay {
    fn report_client_update(&self, _clients: &HashMap<Uuid, api::Client>) {}

    fn report_attribute_updates(&self, _attributes: &HashMap<api::AttrName, api::Attribute>) {}

    fn report_session_update(&self, _state: &api::SessionState) {}
}


#[rstest]
fn test_runtime_client_add_remove() {
    let _rt = Runtime::new();
}