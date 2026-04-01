use std::sync::Arc;

use crate::domain::{
    box_future::BoxFuture, i_message_repository::IMessageRepository,
    mqtt_event_handler::MqttEventHandler,
};
use tracing::{self, info};

pub struct GenericMqttEventHandler {
    message_repository: Arc<dyn IMessageRepository>,
}

impl GenericMqttEventHandler {
    pub fn new(message_repository: Arc<dyn IMessageRepository>) -> Self {
        Self { message_repository }
    }
}

impl MqttEventHandler for GenericMqttEventHandler {
    fn on_message<'a>(&'a self, topic: &'a str, payload: &'a [u8]) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            info!("New message has been published in topic {topic}!");
            self.message_repository.add(topic, payload.to_vec()).await;
        })
    }

    fn on_connect(&self) {
        info!("Client established connection with the broker!");
    }

    fn on_disconnect(&self) {
        info!("Client has been disconnected from the broker!");
    }
}
