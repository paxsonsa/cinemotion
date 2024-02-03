use async_trait::async_trait;

#[async_trait]
trait TakeComponent: Send + Sync {
    async fn new_take(&mut self) -> Result<&Take>;
}
