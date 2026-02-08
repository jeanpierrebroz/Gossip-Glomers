use async_trait::async_trait;
use maelstrom_node::protocol::Message;
use maelstrom_node::{Node, Result, Runtime};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//messages coming into node
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Request {
    Echo { echo: String, msg_id: u64 },
    Init { msg_id: u64 },
}

//messages going out of node
#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum Response {
    EchoOk { echo: String, in_reply_to: u64 },
    InitOk { in_reply_to: u64 },
}

#[derive(Clone, Default)]
struct EchoHandler;

#[async_trait]
impl Node for EchoHandler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {
        let body: Request = req.body.as_obj()?;
        match body {
            Request::Echo { echo, msg_id } => {
                runtime.reply(req, Response::EchoOk { echo, in_reply_to: msg_id }).await
            }
            Request::Init { msg_id } => {
                runtime.reply(req, Response::InitOk { in_reply_to: msg_id }).await
            }
        }
    }
}

pub fn main() -> Result<()> {
    Runtime::init(async {
        let handler = Arc::new(EchoHandler);
        Runtime::new().with_handler(handler).run().await
    })
}