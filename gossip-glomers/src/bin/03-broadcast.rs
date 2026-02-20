use maelstrom_common::{run, HandleMessage, Envelope};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Instant;


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "init")]
    Init { msg_id: Option<usize>, node_id: String },
    #[serde(rename = "init_ok")]
    InitOk { in_reply_to: Option<usize> },

    #[serde(rename = "broadcast")]
    Broadcast { message: i32, msg_id: Option<usize> },
    #[serde(rename = "broadcast_ok")]
    BroadcastOk { in_reply_to: Option<usize> },

    #[serde(rename = "read")]
    Read { msg_id: usize },
    #[serde(rename = "read_ok")]
    ReadOk { messages: Vec<i32>, in_reply_to: Option<usize> },

    #[serde(rename = "topology")]
    Topology { topology: std::collections::HashMap<String, Vec<String>>, msg_id: Option<usize> },
    #[serde(rename = "topology_ok")]
    TopologyOk { in_reply_to: Option<usize>}
}

#[derive(Debug, Default)]
pub struct Broadcast {
    node_id: String,
    messages: HashSet<i32>,
    successfully_sent_messages: HashSet<i32>,
    pending_messages: HashMap<i32, PendingMessage>,
    toplogy: Vec<String>,

}

#[derive(Debug)]
pub struct PendingMessage {
    message: Envelope<Message>,
    time_sent: Instant
}

impl HandleMessage for Broadcast {
    type Message = Message;
    type Error = std::io::Error;

    fn handle_message(
        &mut self,
        msg: Envelope<Self::Message>,
        outbound_msg_tx: std::sync::mpsc::Sender<Envelope<Self::Message>>,
    ) -> Result<(), Self::Error> {
        match msg.body {
            Message::Init { msg_id, ref node_id } => {
                self.node_id = node_id.clone();
                outbound_msg_tx.send(
                    msg.reply(Message::InitOk { in_reply_to: msg_id })
                ).unwrap();
                Ok(())
            },

            Message::Topology { msg_id, ref topology } => {
                if let Some(my_neighbors) = topology.get(&self.node_id) {
                    self.toplogy = my_neighbors.clone(); 
                }
                outbound_msg_tx.send(
                    msg.reply(Message::TopologyOk { in_reply_to: msg_id })
                ).unwrap();
                Ok(())
            },

            Message::Read { msg_id } => {
                let vec = Vec::from_iter(self.messages.iter().cloned());

                outbound_msg_tx.send(
                    msg.reply(Message::ReadOk { messages: vec, in_reply_to: Some(msg_id)})
                ).unwrap();
                Ok(())
            },

            Message::Broadcast { msg_id , message} => {
                let recieved = self.messages.insert(message);

                outbound_msg_tx.send(
                    msg.reply(Message::BroadcastOk { in_reply_to: msg_id })
                ).unwrap();

                if !recieved && !self.successfully_sent_messages.contains(&message) {
                    for i in self.toplogy.clone(){
                        outbound_msg_tx.send(
                            msg.reply(Message::Broadcast { message, msg_id })
                        ).unwrap();
                    }
                }
                Ok(())
            },

            _ => panic!("{}", format!("Unexpected message: {:#?}", serde_json::to_string_pretty(&msg)))
        }
    }
}

fn main() {
    let _ = run(Broadcast::default());
}

