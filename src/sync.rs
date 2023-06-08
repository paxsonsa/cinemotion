use crate::Result;

pub type ResultTx<T> = tokio::sync::oneshot::Sender<Result<T>>;
pub type ResultRx<T> = tokio::sync::oneshot::Receiver<Result<T>>;
pub fn result<T>() -> (ResultTx<T>, ResultRx<T>) {
    tokio::sync::oneshot::channel()
}
