use std::collections::vec_deque;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

#[cfg(test)]
#[path = "test_lib.rs"]
mod test_lib;

pub struct ChannelBuffer<T>
where
    T: Send + Sync + 'static,
{
    buffer: Arc<Mutex<vec_deque::VecDeque<T>>>,
    collector: JoinHandle<()>,
}

impl<T> ChannelBuffer<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(mut channel_rx: mpsc::Receiver<T>, capacity: usize) -> Self {
        let buffer = Arc::new(Mutex::new(vec_deque::VecDeque::with_capacity(capacity)));
        let write_buffer = buffer.clone();
        let collector = tokio::spawn(async move {
            while let Some(item) = channel_rx.recv().await {
                {
                    let mut guard = write_buffer.lock().await;
                    if guard.len() == capacity {
                        guard.pop_front();
                    }
                    guard.push_back(item);
                }
            }
        });
        Self { buffer, collector }
    }

    pub async fn flush(&mut self) -> Vec<T> {
        self.buffer.lock().await.drain(..).collect()
    }
}

impl<T> Drop for ChannelBuffer<T>
where
    T: Send + Sync + 'static,
{
    fn drop(&mut self) {
        self.collector.abort();
    }
}
