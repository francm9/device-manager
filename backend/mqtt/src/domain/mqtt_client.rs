use super::mqtt_error::MqttError;

pub trait MqttClient {
    fn publish(&self, topic: &str, message: &str) -> Result<(), MqttError>;
    fn subscribe(&self, topic: &str) -> Result<(), MqttError>;
    fn disconnect(&self) -> Result<(), MqttError>;
}
