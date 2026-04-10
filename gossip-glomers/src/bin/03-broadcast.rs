use gossip_glomers_rs::io::{Body, Handler, Message};
use gossip_glomers_rs::node::{Node, RpcRetryConfig};
use gossip_glomers_rs::protocol::Protocol;
use gossip_glomers_rs::run;
use std::collections::HashSet;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct BroadcastNode {
    messages: HashSet<usize>,
    topology: Vec<String>,
    last_gossip: Instant,
}

impl Default for BroadcastNode {
    fn default() -> Self {
        Self {
            messages: HashSet::new(),
            topology: Vec::new(),
            last_gossip: Instant::now(),
        }
    }
}

impl Handler<Protocol> for BroadcastNode {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node) {
        let (build_reply, contents) = msg.into_reply(node.get_next_msg_id());
        match contents {
            Protocol::Topology { mut topology } => {
                if let Some(neighbors) = topology.remove(&node.id) {
                    self.topology = neighbors;
                }
                node.send(build_reply(Protocol::TopologyOk));
            }

            Protocol::Read => {
                let msgs: Vec<usize> = self.messages.iter().cloned().collect();
                node.send(build_reply(Protocol::ReadOk {
                    messages: Some(msgs),
                    value: None,
                }));
            }

            Protocol::Broadcast { message } => {
                self.messages.insert(message);
                node.send(build_reply(Protocol::BroadcastOk));
                self.maybe_gossip(node);
            }

            Protocol::BroadcastMany { messages } => {
                for m in messages {
                    self.messages.insert(m);
                }
                node.send(build_reply(Protocol::BroadcastManyOk));
                self.maybe_gossip(node);
            }

            Protocol::BroadcastOk => {}
            Protocol::BroadcastManyOk => {}

            _ => eprintln!("Ignoring unexpected message: {:#?}", contents),
        }
    }
}

impl BroadcastNode {
    fn maybe_gossip(&mut self, node: &Node) {
        if self.last_gossip.elapsed() < Duration::from_millis(100) {
            return;
        }
        self.flush_gossip(node);
    }

    fn flush_gossip(&mut self, node: &Node) {
        if self.messages.is_empty() {
            return;
        }
        let batch: Vec<usize> = self.messages.iter().cloned().collect();
        for neighbor in &self.topology {
            let msg = Message {
                src: node.id.clone(),
                dest: neighbor.clone(),
                body: Body {
                    msg_id: node.get_next_msg_id(),
                    in_reply_to: None,
                    contents: Protocol::BroadcastMany {
                        messages: batch.clone(),
                    },
                },
            };
            node.send(msg);
        }
        self.last_gossip = Instant::now();
    }
}

fn main() {
    run(
        BroadcastNode::default(),
        Some(RpcRetryConfig {
            timeout_ms: 1000,
            max_retries: 0,
            backoff_multiplier: 1,
        }),
    );
}