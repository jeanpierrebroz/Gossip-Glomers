use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::future::Pending;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;

use crate::io::Message;
use crate::protocol::Protocol;

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    msg_counter: Arc<AtomicUsize>,
    pending_message_map: HashMap<usize, PendingMessage>,
    pending_message_heap: BinaryHeap<(Reverse<Instant>, usize)>
}

struct PendingMessage {
    msg: Message<Protocol>,
    retry_count: usize,
    retry_at: Instant
}

pub struct RpcRetryConfig {
    pub timeout_ms: usize,
    pub max_retries: usize,
    pub backoff_multiplier: usize
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

//need to decide if I put the seq-kv method on node or in it's own things
//will probably do node so I can just reuse pending messages
