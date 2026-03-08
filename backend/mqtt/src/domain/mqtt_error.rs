pub enum MqttError {
    ConnectionError(String),
    PublishError(String),
    SubscribeError(String),
    DisconnectionError(String),
}

impl std::fmt::Display for MqttError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MqttError::ConnectionError(msg) => write!(f, "Connection failed: {}", msg),
            MqttError::PublishError(msg) => write!(f, "Publish failed: {}", msg),
            MqttError::SubscribeError(msg) => write!(f, "Subscribe failed: {}", msg),
            MqttError::DisconnectionError(msg) => write!(f, "Disconnect failed: {}", msg),
        }
    }
}
