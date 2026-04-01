use crate::domain::box_future::BoxFuture;

pub trait IMessageRepository: Send + Sync {
    fn add<'a>(&'a self, topic: &'a str, payload: Vec<u8>) -> BoxFuture<'a, ()>;
    fn get_latest<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, Vec<Vec<u8>>>;
}
