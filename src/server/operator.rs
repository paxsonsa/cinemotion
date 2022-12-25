

struct ControllerOperator {
    context: todo!(),
    msg_queue: todo!(),
    controllers: Vec<Box<dyn crate::Controller + Send + Sync>>,
}

impl ControllerOperator {
    pub async fn update(&self) {
        use tracing_futures::Instrument;

        let (time, edit_handle) = self.context.edit().await;
        let mut stream = msg_queue.stream();
        while Some(message) = stream.next().await {
            for controller in controllers {
                controller.update(time, message).instrument(tracing::info_span!(name = controller.name())).await;
            }
        }

        /*

        - Start editing the context which will be a new context tree that is fine to edit in this task.
        - Once the context has been updated and all controllers have run the context is commited.
        - The context is then flushed to the clients as a multi-message where each informer returns the message for the
          state is cares about in the tree.
        */
    }
}