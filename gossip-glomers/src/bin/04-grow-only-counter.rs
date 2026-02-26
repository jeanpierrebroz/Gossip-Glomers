use std::{collections::{HashMap, VecDeque}, sync::mpsc::Sender, thread, time::{Duration, Instant}};
use serde::{Deserialize, Serialize};

/*
Sequential consistency is a strong safety property for concurrent systems. 
Informally, sequential consistency implies that operations appear to take place in some total order, 
and that that order is consistent with the order of operations on each individual process.

Sequential consistency cannot be totally or sticky available; in the event of a network partition, 
some or all nodes will be unable to make progress.

A process in a sequentially consistent system may be far ahead of, or behind, other processes. 
For instance, they may read arbitrarily stale state. However, once a process A has observed some operation 
from process B, it can never observe a state prior to B. 
This, combined with the total ordering property, makes sequential consistency a surprisingly strong model 
for programmers.
*/
use maelstrom_common::{Envelope, HandleMessage, run};

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

    #[serde(rename = "read")]
    Read { msg_id: usize },
    #[serde(rename = "read_ok")]
    ReadOk {
        value: usize,
        in_reply_to: Option<usize>,
    },

    #[serde(rename = "add")]
    Add { 
        msg_id: usize, 
        delta: usize,
    },
    #[serde(rename = "add_ok")]
    AddOk { value: usize },    

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum KVMessage {
    Read {
        key: String,
        msg_id: usize,
    },
    ReadOk {
        value: usize,
        in_reply_to: usize,
    },

    Write {
        key: String,
        value: usize,
        msg_id: usize,
    },
    WriteOk {
        in_reply_to: usize,
    },

    Cas {
        key: String,
        from: usize,
        to: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        create_if_not_exists: Option<bool>,
        msg_id: usize,
    },
    CasOk {
        in_reply_to: usize,
    },

    Error {
        in_reply_to: usize,
        code: usize,
        text: String,
    },
}


#[derive(Debug, Clone)]
pub struct PendingMessage {
    delta: i32,
    time_sent: Instant,
    dest: String,
}

#[derive(Debug, Default)]
pub struct GrowOnlyCounter {
    //keep track of non-acked message (similar to the broadcast challenge), and use a retry loop (also similar to broadcast)
    //send pings to retry messages
    node_id: String,
    counter: usize,
    pending_messages: VecDeque<PendingMessage>,
}


impl HandleMessage for GrowOnlyCounter{
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

            Message::Tick {} => {
                let now = Instant::now();
                let timeout = Duration::from_millis(500);

                for (pending) in &mut self.pending_messages {
                    if now.duration_since(pending.time_sent) >= timeout {
                        pending.time_sent = Instant::now();
                        outbound_msg_tx
                            .send(Envelope {
                                //instead of broadcast, we'd want to requeue the add
                                //also we'd want the adds to apply in a queue 
                                src: self.node_id.clone(),
                                dest: pending.dest.clone(),
                                body: Message:: {
                                    message: pending.,
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



pub fn main() {
    let _ = run(GrowOnlyCounter::default());
}