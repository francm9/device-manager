use std::sync::Arc;

use rumqttc::{Client, MqttOptions};

use crate::domain::connect_service_trait::ConnectServiceTrait;
use crate::domain::mqtt_client::MqttClient;
use crate::domain::mqtt_error::MqttError;

use super::rumqttc_client::RumqttcClient;

pub struct RumqttcConnectionService {
    id: String,
    host: String,
    port: u16,
}

impl ConnectServiceTrait for RumqttcConnectionService {
    fn connect(&self) -> Result<Arc<dyn MqttClient>, MqttError> {
        let options = MqttOptions::new(self.id.clone(), self.host.clone(), self.port);
        let (client, mut connection) = Client::new(options, 100);

        for (_, notification) in connection.iter().enumerate() {
            println!("Notification = {:?}", notification);
        }

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
