use rumqttc::{AsyncClient, MqttOptions};
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use crate::domain::i_message_repository::IMessageRepository;
use crate::domain::mqtt_client::MqttClient;
use crate::domain::mqtt_event_handler::MqttEventHandler;
use crate::infrastructure::generic_mqtt_event_handler::GenericMqttEventHandler;
use crate::infrastructure::in_memory_message_repository::InMemoryMessageRepository;
use crate::infrastructure::rumqttc_client::RumqttcClient;
use crate::infrastructure::rumqttc_event_loop::RumqttcEventLoop;

pub mod domain;
pub mod infrastructure;

struct MqttState {
    client: Arc<dyn MqttClient>,
    repository: Arc<dyn IMessageRepository>,
}

impl std::fmt::Debug for MqttState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MqttState").finish()
    }
}

static MQTT_STATE: OnceLock<MqttState> = OnceLock::new();

fn state() -> &'static MqttState {
    MQTT_STATE
        .get()
        .expect("MQTT not connected, call connect() first")
}

/// Connects to the MQTT broker.
///
/// # Panics
///
/// Panics if `connect()` has already been called.
pub fn connect(host: &str, port: u16, client_id: &str) {
    let mut options = MqttOptions::new(client_id, host, port);
    options.set_keep_alive(Duration::from_secs(30));

    let (async_client, event_loop) = AsyncClient::new(options, 10);

    let repository: Arc<dyn IMessageRepository> = Arc::new(InMemoryMessageRepository::default());

    let handler: Arc<dyn MqttEventHandler> =
        Arc::new(GenericMqttEventHandler::new(Arc::clone(&repository)))
            as Arc<dyn MqttEventHandler>;

    let client: Arc<dyn MqttClient> =
        Arc::new(RumqttcClient::new(async_client)) as Arc<dyn MqttClient>;

    MQTT_STATE
        .set(MqttState {
            client: Arc::clone(&client),
            repository: Arc::clone(&repository),
        })
        .expect("connect() already called");

    tokio::spawn(async move {
        RumqttcEventLoop::new(event_loop, handler).run().await;
    });
}

pub async fn publish(topic: &str, message: &str) {
    let _ = state().client.publish(topic, message).await;
}

pub async fn subscribe(topic: &str) {
    let _ = state().client.subscribe(topic).await;
}

pub async fn get_new_topic_messages(topic: &str) -> Vec<Vec<u8>> {
    state().repository.get_latest(topic).await
}

pub async fn disconnect() {
    let _ = state().client.disconnect().await;
}
