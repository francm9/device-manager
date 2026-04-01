use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::sync::RwLock;
use tracing::warn;

use crate::domain::{box_future::BoxFuture, i_message_repository::IMessageRepository};

pub struct InMemoryMessageRepository {
    messages: Arc<RwLock<HashMap<String, VecDeque<Vec<u8>>>>>,
}

impl Default for InMemoryMessageRepository {
    fn default() -> Self {
        Self {
            messages: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl IMessageRepository for InMemoryMessageRepository {
    fn add<'a>(&'a self, topic: &'a str, payload: Vec<u8>) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let mut store = self.messages.write().await;
            let queue = store
                .entry(topic.to_string())
                .or_insert_with(|| VecDeque::with_capacity(100));

            if queue.len() == queue.capacity() {
                warn!("Queue is full!");
                queue.pop_front();
            }
            queue.push_back(payload);
            drop(store);
        })
    }

    fn get_latest<'a>(&'a self, topic: &'a str) -> BoxFuture<'a, Vec<Vec<u8>>> {
        Box::pin(async move {
            let mut store = self.messages.write().await;
            let result = store
                .get_mut(topic)
                .map_or_else(Vec::new, |q| q.drain(..).collect());
            drop(store);
            result
        })
    }
}
