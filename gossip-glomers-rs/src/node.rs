use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Instant, Duration};

use crate::io::{Message, send};
use crate::protocol::Protocol;

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    msg_counter: Arc<AtomicUsize>,
    pending_message_map: Arc<Mutex<HashMap<usize, PendingMessage>>>,
    pending_message_heap: Arc<Mutex<BinaryHeap<(Reverse<Instant>, usize)>>>,
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
            pending_message_heap: Arc::new(Mutex::new(BinaryHeap::new())),
            pending_message_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn get_next_msg_id(&self) -> usize {
        self.msg_counter.fetch_add(1, Ordering::SeqCst)
    }
    
    fn start_callback_loop(config: Arc<RpcRetryConfig>, pending: Arc<Mutex<HashMap<usize, PendingMessage>>>, heap: Arc<Mutex<BinaryHeap<(Reverse<Instant>, usize)>>>) {
        thread::spawn(move || {
            loop {
                let sleep_duration = {
                    let mut heap = heap.lock().unwrap();
                    if let Some((Reverse(retry_at), msg_id)) = heap.peek().copied() {
                        let now = Instant::now();
                        if retry_at <= now {
                            heap.pop();
                            let mut pending = pending.lock().unwrap();
                            if let Some(entry) = pending.get_mut(&msg_id) {
                                if entry.retry_count >= config.max_retries {
                                    eprintln!("RPC {} to {} timed out", msg_id, entry.msg.dest);
                                    pending.remove(&msg_id);
                                } else {
                                    entry.retry_count += 1;
                                    let backoff = config.timeout_ms * config.backoff_multiplier.pow(entry.retry_count as u32);
                                    entry.retry_at = now + Duration::from_millis(backoff as u64);
                                    send(&entry.msg);
                                    heap.push((Reverse(entry.retry_at), msg_id));
                                }
                            }
                            Duration::from_millis(0)
                        } else {
                            retry_at - now
                        }
                    } else {
                        Duration::from_millis(50) // nothing pending, check back soon
                    }
                };
                thread::sleep(sleep_duration);
            }
        });
    }
}
