use rumqttc::{AsyncClient, MqttOptions};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info};

use self::domain::mqtt_client::MqttClient;
use self::infrastructure::generic_mqtt_event_handler::GenericMqttEventHandler;
use self::infrastructure::rumqttc_client::RumqttcClient;
use self::infrastructure::rumqttc_event_loop::RumqttcEventLoop;

pub mod application;
pub mod domain;
pub mod infrastructure;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("MQTT Service loading...");

    let mut options = MqttOptions::new("prueba", "localhost", 1883);
    options.set_keep_alive(Duration::from_secs(30));

    let (async_client, event_loop) = AsyncClient::new(options, 10);

    let handler = Arc::new(GenericMqttEventHandler::new());
    let client: Arc<dyn MqttClient> = Arc::new(RumqttcClient::new(async_client));

    let event_loop_handle = tokio::spawn(RumqttcEventLoop::new(event_loop, handler).run());

    let _ = client.subscribe("sensors").await;
    let _ = client.publish("status", "online").await;

    if let Err(e) = event_loop_handle.await {
        error!("Event loop panicked: {e}");
    }
}
