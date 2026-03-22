use std::sync::Arc;

use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions};

use crate::domain::connect_service_trait::ConnectServiceTrait;
use crate::domain::mqtt_client::MqttClient;
use crate::domain::mqtt_error::MqttError;

use super::rumqttc_client::RumqttcClient;

pub struct RumqttcConnectionService {
    id: String,
    host: String,
    port: u16,
}

#[async_trait]
impl ConnectServiceTrait for RumqttcConnectionService {
    async fn connect(&self) -> Result<Arc<dyn MqttClient>, MqttError> {
        let options = MqttOptions::new(self.id.clone(), self.host.clone(), self.port);
        let (client, mut eventloop) = AsyncClient::new(options, 100);

        let notification = eventloop.poll().await.unwrap();
        println!("Notification = {:?}", notification);

        Ok(Arc::new(RumqttcClient::new(client)))
    }
}

impl RumqttcConnectionService {
    pub fn new(id: &str, host: &str, port: u16) -> Self {
        Self {
            id: id.to_string(),
            host: host.to_string(),
            port,
        }
    }
}
