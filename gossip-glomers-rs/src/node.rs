use crate::io::Message;
use crate::protocol::Protocol;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    msg_counter: Arc<AtomicUsize>,
}

impl Node {
    pub fn new(id: String, node_ids: Vec<String>) -> Self {
        Self {
            id,
            node_ids,
            msg_counter: Arc::new(AtomicUsize::new(1)),
        }
    }
    pub fn get_next_msg_id(&self) -> usize {
        self.msg_counter.fetch_add(1, Ordering::SeqCst)
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            node_ids: self.node_ids.clone(),
            msg_counter: Arc::clone(&self.msg_counter),
        }
    }
}
