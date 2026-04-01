use crate::domain::mqtt_error::MqttError;
use crate::domain::{box_future::BoxFuture, mqtt_client::MqttClient};
use rumqttc::{AsyncClient, QoS};

pub struct RumqttcClient {
    client: AsyncClient,
}

impl RumqttcClient {
    #[must_use]
    pub const fn new(client: AsyncClient) -> Self {
        Self { client }
    }
}

impl MqttClient for RumqttcClient {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        message: &'a str,
    ) -> BoxFuture<'a, Result<(), MqttError>> {
        Box::pin(async move {
            self.client
                .publish(topic, QoS::AtLeastOnce, false, message)
                .await
                .map_err(|_| {
                    MqttError::PublishError(
                        "An error ocurred while publishing the message".to_string(),
                    )
                })
        })
    }

    fn subscribe<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, Result<(), MqttError>> {
        Box::pin(async move {
            self.client
                .subscribe(topic, QoS::AtMostOnce)
                .await
                .map_err(|_| {
                    MqttError::SubscribeError(
                        "An error ocurred while suscribing to a topic".to_string(),
                    )
                })
        })
    }

    fn disconnect(&self) -> BoxFuture<'_, Result<(), MqttError>> {
        Box::pin(async move {
            self.client.disconnect().await.map_err(|_| {
                MqttError::DisconnectionError(
                    "An error ocurred while disconnecting from the broker".to_string(),
                )
            })
        })
    }
}
