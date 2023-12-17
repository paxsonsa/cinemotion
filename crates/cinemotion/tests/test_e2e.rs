use cinemotion::*;

mod common;

#[tokio::test]
async fn test_session_initialize() {
    let (engine, _spy) = common::make_engine();
}
