use rumqttc::{Client, QoS};

use crate::domain::mqtt_client::MqttClient;
use crate::domain::mqtt_error::MqttError;

pub struct RumqttcClient {
    client: Client,
}

impl RumqttcClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl MqttClient for RumqttcClient {
    fn publish(&self, topic: &str, message: &str) -> Result<(), MqttError> {
        let _ = self
            .client
            .publish(topic, QoS::AtLeastOnce, false, message)
            .map_err(|_| MqttError::PublishError);

        Ok(())
    }

    fn subscribe(&self, topic: &str) -> Result<(), MqttError> {
        let _ = self
            .client
            .subscribe(topic, QoS::AtLeastOnce)
            .map_err(|_| MqttError::SubscribeError);

        Ok(())
    }

    fn disconnect(&self) -> Result<(), MqttError> {
        let _ = self
            .client
            .disconnect()
            .map_err(|_| MqttError::DisconnectionError);

        Ok(())
    }
}
