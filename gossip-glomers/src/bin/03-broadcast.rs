use maelstrom_common::{Envelope, HandleMessage, run};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "init")]
    Init {
        msg_id: Option<usize>,
        node_id: String,
    },
    #[serde(rename = "init_ok")]
    InitOk { in_reply_to: Option<usize> },

    #[serde(rename = "broadcast")]
    Broadcast { message: i32, msg_id: Option<usize> },
    #[serde(rename = "broadcast_ok")]
    BroadcastOk { in_reply_to: Option<usize> },

    #[serde(rename = "read")]
    Read { msg_id: usize },
    #[serde(rename = "read_ok")]
    ReadOk {
        messages: Vec<i32>,
        in_reply_to: Option<usize>,
    },

    #[serde(rename = "topology")]
    Topology {
        topology: HashMap<String, Vec<String>>,
        msg_id: Option<usize>,
    },
    #[serde(rename = "topology_ok")]
    TopologyOk { in_reply_to: Option<usize> },

    #[serde(rename = "tick")]
    Tick {},
}

#[derive(Debug, Clone)]
pub struct PendingMessage {
    message: i32,
    time_sent: Instant,
    dest: String,
}

#[derive(Debug, Default)]
pub struct Broadcast {
    node_id: String,
    messages: HashSet<i32>,
    pending_messages: HashMap<usize, PendingMessage>,
    topology: Vec<String>,
    counter: usize,
}

impl HandleMessage for Broadcast {
    type Message = Message;
    type Error = std::io::Error;

    fn handle_message(
        &mut self,
        msg: Envelope<Self::Message>,
        outbound_msg_tx: Sender<Envelope<Self::Message>>,
    ) -> Result<(), Self::Error> {
        match msg.body {
            Message::Init {
                msg_id,
                ref node_id,
            } => {
                self.node_id = node_id.clone();
                self.counter = 0;

                let node_id_clone = self.node_id.clone();
                let outbound_clone = outbound_msg_tx.clone();

                thread::spawn(move || {
                    loop {
                        thread::sleep(Duration::from_millis(500));
                        let _ = outbound_clone.send(Envelope {
                            src: node_id_clone.clone(),
                            dest: node_id_clone.clone(),
                            body: Message::Tick {},
                        });
                    }
                });

                outbound_msg_tx
                    .send(msg.reply(Message::InitOk {
                        in_reply_to: msg_id,
                    }))
                    .unwrap();

                Ok(())
            }

            Message::Topology {
                msg_id,
                ref topology,
            } => {
                if let Some(my_neighbors) = topology.get(&self.node_id) {
                    self.topology = my_neighbors.clone();
                }
                outbound_msg_tx
                    .send(msg.reply(Message::TopologyOk {
                        in_reply_to: msg_id,
                    }))
                    .unwrap();
                Ok(())
            }

            Message::Read { msg_id } => {
                let vec = Vec::from_iter(self.messages.iter().cloned());
                outbound_msg_tx
                    .send(msg.reply(Message::ReadOk {
                        messages: vec,
                        in_reply_to: Some(msg_id),
                    }))
                    .unwrap();
                Ok(())
            }

            Message::Broadcast { msg_id, message } => {
                let is_new = self.messages.insert(message);

                outbound_msg_tx
                    .send(msg.reply(Message::BroadcastOk {
                        in_reply_to: msg_id,
                    }))
                    .unwrap();

                if is_new {
                    for neighbor in &self.topology {
                        let id = self.counter;
                        self.counter += 1;

                        let pending = PendingMessage {
                            message,
                            time_sent: Instant::now(),
                            dest: neighbor.clone(),
                        };
                        self.pending_messages.insert(id, pending);

                        outbound_msg_tx
                            .send(Envelope {
                                src: self.node_id.clone(),
                                dest: neighbor.clone(),
                                body: Message::Broadcast {
                                    message,
                                    msg_id: Some(id),
                                },
                            })
                            .unwrap();
                    }
                }
                Ok(())
            }

            Message::BroadcastOk { in_reply_to } => {
                if let Some(id) = in_reply_to {
                    self.pending_messages.remove(&id);
                }
                Ok(())
            }

            Message::Tick {} => {
                let now = Instant::now();
                let timeout = Duration::from_millis(500);

                for (id, pending) in &mut self.pending_messages {
                    if now.duration_since(pending.time_sent) >= timeout {
                        pending.time_sent = Instant::now();
                        outbound_msg_tx
                            .send(Envelope {
                                src: self.node_id.clone(),
                                dest: pending.dest.clone(),
                                body: Message::Broadcast {
                                    message: pending.message,
                                    msg_id: Some(*id),
                                },
                            })
                            .unwrap();
                    }
                }
                Ok(())
            }

            _ => panic!(
                "{}",
                format!(
                    "Unexpected message: {:#?}",
                    serde_json::to_string_pretty(&msg)
                )
            ),
        }
    }
}

fn main() {
    let _ = run(Broadcast::default());
}
