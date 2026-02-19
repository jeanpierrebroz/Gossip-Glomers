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
    #[serde(rename = "echo")]
    Echo {
        echo: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        msg_id: Option<usize>
    },
    #[serde(rename = "init_ok")]
    InitOk {
        #[serde(skip_serializing_if = "Option::is_none")]
        in_reply_to: Option<usize>
    },
    #[serde(rename = "echo_ok")]
    EchoOk {
        echo: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        in_reply_to: Option<usize>
    },
}

#[derive(Debug, Default)]
pub struct Echo {
    node_id: Option<String>,
}

impl HandleMessage for Echo {
    type Message = Message;
    type Error = std::io::Error;

    fn handle_message(
        &mut self,
        msg: Envelope<Self::Message>,
        outbound_msg_tx: std::sync::mpsc::Sender<Envelope<Self::Message>>,
    ) -> Result<(), Self::Error> {
        match msg.body {
            Message::Init { msg_id, ref node_id } => {
                self.node_id = Some(node_id.clone());
                outbound_msg_tx.send(
                    msg.reply(Message::InitOk { in_reply_to: msg_id })
                ).unwrap();
                Ok(())
            },
            Message::Echo { ref echo, msg_id } => {
                outbound_msg_tx.send(
                    msg.reply(
                    Message::EchoOk { echo: echo.to_owned(), in_reply_to: msg_id }
                    )
                ).unwrap();
                Ok(())
            },
            _ => panic!("{}", format!("Unexpected message: {:#?}", serde_json::to_string_pretty(&msg)))
        }
    }
}

fn main() {
    let _ = run(Echo::default());
}

