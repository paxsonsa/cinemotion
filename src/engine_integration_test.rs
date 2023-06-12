use super::*;

#[tokio::test]
async fn test_client_intiatilization() {
    let shutdown_channel = tokio::sync::mpsc::channel(1);
    let (tick_control, ticker) = engine::TickControl::channel();
    let (mut service, transport) = engine::Service::new();
    let mut controller = engine::EngineController::new(transport, shutdown_channel.1, tick_control);

    let handle = tokio::spawn(async move {
        let _ = controller.run().await;
    });

    let command = api::models::Client::new(0, "clientA".to_string());
    let command = api::Command::SetClient(command);

    service
        .enqueue_command(command)
        .await
        .expect("Failed to enqueue command");
    ticker.send(()).await.unwrap();
    let state = service.recv_state_update().await.unwrap();

    // TODO Test Client Additional
    assert_eq!(state.clients.len(), 1);
    assert_eq!(state.clients[&0].id, 0);
    assert_eq!(state.clients[&0].name, "clientA");

    shutdown_channel.0.send(()).await.unwrap();
    handle.await.unwrap();
}
