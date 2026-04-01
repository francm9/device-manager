use super::mqtt_error::MqttError;
use crate::domain::box_future::BoxFuture;

pub trait MqttClient: Send + Sync {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        message: &'a str,
    ) -> BoxFuture<'a, Result<(), MqttError>>;
    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, Result<(), MqttError>>;
    fn disconnect(&self) -> BoxFuture<'_, Result<(), MqttError>>;
}
