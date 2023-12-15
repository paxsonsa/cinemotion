use std::sync::Mutex;

use arc_swap::ArcSwapOption;
use async_trait::async_trait;
use cinemotion;
use cinemotion::commands::Event;
use cinemotion::session::SendHandlerFn;

struct MockSession {
    pub send_fn: ArcSwapOption<Mutex<SendHandlerFn>>,
}

#[async_trait]
impl cinemotion::session::SessionAgent for MockSession {
    async fn initialize(&mut self, send_fn: SendHandlerFn) {
        todo!()
    }

    async fn receive(&mut self, event: Event) {
        todo!()
    }

    async fn close(&mut self) {
        todo!()
    }
}
