use std::sync::Arc;

use crate::domain::connect_service_trait::ConnectServiceTrait;
use crate::domain::mqtt_client::MqttClient;
use crate::domain::mqtt_error::MqttError;

pub struct ConnectService {
    connection_service: Arc<dyn ConnectServiceTrait>,
}

impl ConnectService {
    pub fn new(connection_service: Arc<dyn ConnectServiceTrait>) -> Self {
        Self { connection_service }
    }

    pub async fn connect(&self) -> Result<Arc<dyn MqttClient>, MqttError> {
        self.connection_service.connect().await
    }
}
