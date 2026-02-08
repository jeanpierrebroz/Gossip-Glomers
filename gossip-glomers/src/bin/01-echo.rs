use async_trait::async_trait;
use maelstrom_node::protocol::Message;
use maelstrom_node::{Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
        msg_id: usize,
    },
    EchoOk {
        echo: String,
        in_reply_to: usize,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
        msg_id: usize,
    },
    InitOk {
        in_reply_to: usize,
    },
}

#[derive(Default)]
struct EchoHandler;

#[async_trait]
impl Node for EchoHandler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let body: Payload = match req.body.as_obj() {
            Ok(b) => b,
            Err(e) => {
                eprintln!("ERROR: Deserialization failed: {}", e);
                return Ok(());
            }
        };

        match body {
            Payload::Init { msg_id, .. } => {
                let reply = Payload::InitOk {
                    in_reply_to: msg_id,
                };
                runtime.reply(req, reply).await?;
            }
            Payload::Echo { echo, msg_id } => {
                let reply = Payload::EchoOk {
                    echo,
                    in_reply_to: msg_id,
                };
                runtime.reply(req, reply).await?;
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    Runtime::init(async {
        let handler = Arc::new(EchoHandler::default());
        Runtime::new().with_handler(handler).run().await
    })
}