use maelstrom_common::{run, HandleMessage, Envelope};
use serde::{Deserialize, Serialize};
use core::panic;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "init")]
    Init {
        #[serde(skip_serializing_if = "Option::is_none")]
        msg_id: Option<usize>,
        node_id: String
    },
    #[serde(rename = "init_ok")]
    InitOk {
        #[serde(skip_serializing_if = "Option::is_none")]
        in_reply_to: Option<usize>
    },
    #[serde(rename = "generate_ok")]
    GenerateOk {
        #[serde(skip_serializing_if = "Option::is_none")]
        in_reply_to: Option<usize>,
        id: String,
    },
    #[serde(rename = "generate")]
    Generate {
        msg_id: usize,
    },
}

#[derive(Debug, Default)]
pub struct UID {
    node_id: String,
    counter: u32
}

impl HandleMessage for UID {
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

            Message::Generate { msg_id } => {
                let id = format!("{}-{}", self.node_id, self.counter);
                outbound_msg_tx.send(
                    msg.reply(
                    Message::GenerateOk { id: id, in_reply_to: Some(msg_id) }
                    )
                ).unwrap();
                self.counter+=1;
                Ok(())
            },
            _ => panic!("{}", format!("Unexpected message: {:#?}", serde_json::to_string_pretty(&msg)))
        }
    }
}

fn main() {
    let _ = run(UID::default());
}

