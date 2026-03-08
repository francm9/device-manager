use std::sync::Arc;

use super::mqtt_client::MqttClient;
use super::mqtt_error::MqttError;

pub trait ConnectServiceTrait {
    fn connect(&self) -> Result<Arc<dyn MqttClient>, MqttError>;
}
