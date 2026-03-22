use std::sync::Arc;

use async_trait::async_trait;

use super::mqtt_client::MqttClient;
use super::mqtt_error::MqttError;

#[async_trait]
pub trait ConnectServiceTrait {
    async fn connect(&self) -> Result<Arc<dyn MqttClient>, MqttError>;
}
