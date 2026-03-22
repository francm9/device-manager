use async_trait::async_trait;

#[async_trait]
pub trait MqttEventHandler: Send + Sync {
    async fn on_message(&self, topic: &str, payload: &[u8]);
    async fn on_connect(&self);
    async fn on_disconnect(&self);
}
