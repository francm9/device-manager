use crate::domain::mqtt_client::MqttClient;
use crate::domain::mqtt_error::MqttError;
use async_trait::async_trait;
use rumqttc::{AsyncClient, QoS};

pub struct RumqttcClient {
    client: AsyncClient,
}

impl RumqttcClient {
    pub fn new(client: AsyncClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MqttClient for RumqttcClient {
    async fn publish(&self, topic: &str, message: &str) -> Result<(), MqttError> {
        self.client
            .publish(topic, QoS::AtLeastOnce, false, message)
            .await
            .map_err(|_| {
                MqttError::PublishError("An error ocurred while publishing the message".to_string())
            })
    }

    async fn subscribe(&self, topic: &str) -> Result<(), MqttError> {
        self.client
            .subscribe(topic, QoS::AtMostOnce)
            .await
            .map_err(|_| {
                MqttError::SubscribeError(
                    "An error ocurred while suscribing to a topic".to_string(),
                )
            })
    }

    async fn disconnect(&self) -> Result<(), MqttError> {
        self.client.disconnect().await.map_err(|_| {
            MqttError::DisconnectionError(
                "An error ocurred while disconnecting from the broker".to_string(),
            )
        })
    }
}
