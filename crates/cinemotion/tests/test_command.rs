use cinemotion::{commands, Event, EventBody, Message};
mod common;

#[tokio::test]
async fn test_echo_command() {
    let (builder, spy) = common::make_engine();
    let mut engine = builder.build().expect("engine should build successfully");

    let request = Message::with_command(
        1,
        commands::PeerCommand::from(commands::Echo::from("hello".to_string())),
    );
    engine.apply(request).await.expect("failed to apply engine");

    let spy = spy.session_component.lock().unwrap();
    assert!(spy.send_called);
    assert_eq!(spy.send_called_args.len(), 1);
    let event = spy.send_called_args[0].clone();

    // The same echo message should be broadcast back to the session
    assert_eq!(
        event,
        Event::new(1, EventBody::Echo("hello".to_string().into()))
    );
}
