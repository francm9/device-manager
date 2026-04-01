use crate::domain::box_future::BoxFuture;

pub trait MqttEventHandler: Send + Sync {
    fn on_message<'a>(&'a self, topic: &'a str, payload: &'a [u8]) -> BoxFuture<'a, ()>;
    fn on_connect(&self);
    fn on_disconnect(&self);
}
