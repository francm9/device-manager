pub enum MqttError {
    ConnectionError(String),
    PublishError(String),
    SubscribeError(String),
    DisconnectionError(String),
}

impl std::fmt::Display for MqttError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ConnectionError(msg) => write!(f, "Connection failed: {msg}"),
            Self::PublishError(msg) => write!(f, "Publish failed: {msg}"),
            Self::SubscribeError(msg) => write!(f, "Subscribe failed: {msg}"),
            Self::DisconnectionError(msg) => write!(f, "Disconnect failed: {msg}"),
        }
    }
}
