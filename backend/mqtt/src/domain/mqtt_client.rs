use super::mqtt_error::MqttError;
use async_trait::async_trait;

#[async_trait]
pub trait MqttClient: Send + Sync {
    async fn publish(&self, topic: &str, message: &str) -> Result<(), MqttError>;
    async fn subscribe(&self, topic: &str) -> Result<(), MqttError>;
    async fn disconnect(&self) -> Result<(), MqttError>;
}
