use crate::domain::mqtt_event_handler::MqttEventHandler;
use async_trait::async_trait;
use tracing::{self, info};

pub struct GenericMqttEventHandler {}

impl GenericMqttEventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MqttEventHandler for GenericMqttEventHandler {
    async fn on_message(&self, topic: &str, _payload: &[u8]) {
        info!("New message has been published in topic {topic}!");
    }

    async fn on_connect(&self) {
        info!("Client established connection with the broker!");
    }

    async fn on_disconnect(&self) {
        info!("Client has been disconnected from the broker!");
    }
}
