use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    msg_counter: Arc<AtomicUsize>,
    //probably add pending messages here?
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
