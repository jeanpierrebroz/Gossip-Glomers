use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Instant, Duration};

use crate::io::{Message};
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

#[derive(Clone)]
pub struct RpcRetryConfig {
    pub timeout_ms: usize,
    pub max_retries: usize,
    pub backoff_multiplier: usize
}

impl Default for RpcRetryConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 1000,
            max_retries: 3,
            backoff_multiplier: 2,
        }
    }
}


impl Node {
    pub fn new(id: String, node_ids: Vec<String>, config: RpcRetryConfig) -> Self {
        let pending_message_heap = Arc::new(Mutex::new(BinaryHeap::new()));
        let pending_message_map = Arc::new(Mutex::new(HashMap::new()));
        
        let node = Self {
            id,
            node_ids,
            msg_counter: Arc::new(AtomicUsize::new(1)),
            pending_message_heap: Arc::clone(&pending_message_heap),
            pending_message_map: Arc::clone(&pending_message_map),
        };
                
        Node::start_callback_loop(config, Arc::clone(&pending_message_map), Arc::clone(&pending_message_heap));
        node
    }
    pub fn get_next_msg_id(&self) -> usize {
        self.msg_counter.fetch_add(1, Ordering::SeqCst)
    }
    
    fn start_callback_loop(config: RpcRetryConfig, pending: Arc<Mutex<HashMap<usize, PendingMessage>>>, heap: Arc<Mutex<BinaryHeap<(Reverse<Instant>, usize)>>>) {
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
                                    write_message(&entry.msg);
                                    heap.push((Reverse(entry.retry_at), msg_id));
                                }
                            }
                            Duration::from_millis(0)
                        } else {
                            retry_at - now
                        }
                    } else {
                        Duration::from_millis(50) 
                    }
                };
                thread::sleep(sleep_duration);
            }
        });
    }
    
    //how do I make sure the node's send method pops/adds messages?
    // maybe one send method on node that adds, then another that actually writes?
    pub fn send(&self, msg: &Message<Protocol>) {
        let mut map = self.pending_message_map.lock().unwrap();
        let mut heap = self.pending_message_heap.lock().unwrap();
        
        let copy = msg.clone();
        let retry_at = Instant::now() + Duration::from_millis(500);
                
        let pending_msg = PendingMessage {msg: copy, retry_count: 0, retry_at: retry_at };
        
        map.insert(msg.body.msg_id, pending_msg);
        heap.push(());
    }
    
}

fn write_message(msg: &Message<Protocol>) {
    let out = std::io::stdout().lock();
    serde_json::to_writer(out, msg).unwrap();
    println!();
}
//TODO: SeqKV and write a lot of tests