
use async_trait::async_trait;

#[async_trait]
pub trait HostedService {
  async fn start(&mut self);
  async fn stop(&mut self);
}