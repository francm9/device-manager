use rumqttc::{AsyncClient, MqttOptions};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

use crate::domain::i_message_repository::IMessageRepository;
use crate::domain::mqtt_event_handler::MqttEventHandler;
use crate::infrastructure::in_memory_message_repository::InMemoryMessageRepository;

use self::domain::mqtt_client::MqttClient;
use self::infrastructure::generic_mqtt_event_handler::GenericMqttEventHandler;
use self::infrastructure::rumqttc_client::RumqttcClient;
use self::infrastructure::rumqttc_event_loop::RumqttcEventLoop;

pub mod domain;
pub mod infrastructure;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("MQTT Service loading...");

    let mut options = MqttOptions::new("prueba", "localhost", 1883);
    options.set_keep_alive(Duration::from_secs(30));

    let (async_client, event_loop) = AsyncClient::new(options, 10);

    let message_repository: Arc<dyn IMessageRepository> =
        Arc::new(InMemoryMessageRepository::default());
    let handler: Arc<dyn MqttEventHandler> =
        Arc::new(GenericMqttEventHandler::new(message_repository)) as Arc<dyn MqttEventHandler>;
    let client: Arc<dyn MqttClient> =
        Arc::new(RumqttcClient::new(async_client)) as Arc<dyn MqttClient>;

    let event_loop_handle = tokio::spawn(RumqttcEventLoop::new(event_loop, handler).run());

    let _ = client.subscribe("status").await;
    let _ = client.publish("sensors", "online").await;

    if let Err(e) = event_loop_handle.await {
        error!("Event loop panicked: {e}");
    }
}
