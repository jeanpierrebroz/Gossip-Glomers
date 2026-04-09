use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::io::Message;
use crate::protocol::Protocol;

struct PendingMessages {
    map: HashMap<usize, PendingMessage>,
    heap: BinaryHeap<(Reverse<Instant>, usize)>,
}

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    msg_counter: Arc<AtomicUsize>,
    config: RpcRetryConfig,
    pending: Arc<Mutex<PendingMessages>>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            node_ids: self.node_ids.clone(),
            msg_counter: Arc::clone(&self.msg_counter),
            config: self.config.clone(),
            pending: Arc::clone(&self.pending),
        }
    }
}

struct PendingMessage {
    msg: Message<Protocol>,
    retry_count: usize,
    retry_at: Instant,
}

#[derive(Clone)]
pub struct RpcRetryConfig {
    pub timeout_ms: usize,
    pub max_retries: usize,
    pub backoff_multiplier: usize,
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
        let pending = Arc::new(Mutex::new(PendingMessages {
            map: HashMap::new(),
            heap: BinaryHeap::new(),
        }));

        let node = Self {
            id,
            node_ids,
            msg_counter: Arc::new(AtomicUsize::new(1)),
            config: config.clone(),
            pending: Arc::clone(&pending),
        };

        Node::start_callback_loop(config, Arc::clone(&pending));
        node
    }

    pub fn get_next_msg_id(&self) -> usize {
        self.msg_counter.fetch_add(1, Ordering::SeqCst)
    }

    fn start_callback_loop(config: RpcRetryConfig, pending: Arc<Mutex<PendingMessages>>) {
        thread::spawn(move || {
            loop {
                let sleep_duration = {
                    let mut pending = pending.lock().unwrap();
                    if let Some((Reverse(retry_at), msg_id)) = pending.heap.peek().copied() {
                        let now = Instant::now();
                        if retry_at <= now {
                            pending.heap.pop();

                            if !pending.map.contains_key(&msg_id) {
                                continue;
                            }

                            let timed_out = pending
                                .map
                                .get(&msg_id)
                                .map(|e| e.retry_count >= config.max_retries)
                                .unwrap_or(false);

                            if timed_out {
                                if let Some(entry) = pending.map.remove(&msg_id) {
                                    eprintln!("RPC {} to {} timed out", msg_id, entry.msg.dest);
                                }
                            } else if let Some(entry) = pending.map.get_mut(&msg_id) {
                                entry.retry_count += 1;
                                let backoff = config.timeout_ms
                                    * config.backoff_multiplier.pow(entry.retry_count as u32);
                                entry.retry_at = now + Duration::from_millis(backoff as u64);
                                write_message(&entry.msg);
                                let new_retry_at = entry.retry_at;
                                pending.heap.push((Reverse(new_retry_at), msg_id));
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

    pub fn send(&self, msg: Message<Protocol>) {
        let mut pending = self.pending.lock().unwrap();
        let msg_id = msg.body.msg_id;

        write_message(&msg);

        if msg.body.in_reply_to.is_some() {
            return; 
        }
        
            let retry_at = Instant::now() + Duration::from_millis(self.config.timeout_ms as u64);
            pending.heap.push((Reverse(retry_at), msg_id));
            pending.map.insert(
                msg_id,
                PendingMessage {
                    msg,
                    retry_count: 0,
                    retry_at,
                },
            );
    }

    pub fn ack(&self, msg_id: usize) {
        let mut pending = self.pending.lock().unwrap();
        pending.map.remove(&msg_id);
    }

    pub fn pending_count(&self) -> usize {
        self.pending.lock().unwrap().map.len()
    }

    pub fn heap_count(&self) -> usize {
        self.pending.lock().unwrap().heap.len()
    }
}

fn write_message(msg: &Message<Protocol>) {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer(&mut out, msg).unwrap();
    writeln!(out).unwrap();
}
