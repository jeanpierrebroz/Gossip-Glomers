pub mod io;
pub mod node;
pub mod protocol;

use io::{Handler, parse};
use io::Message;
use node::{Node, RpcRetryConfig};
use protocol::Protocol;
use std::io::BufRead;

pub fn run<H: Handler>(mut handler: H, rpc_retry_config: Option<RpcRetryConfig>) {
    let stdin = std::io::stdin();
    let mut node: Option<Node> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let msg: Message<Protocol> = parse(&line);

        match msg.body.contents {
            Protocol::Init { ref node_id, ref node_ids } => {
                let config = rpc_retry_config.clone().unwrap_or_default();
                let n = Node::new(node_id.clone(), node_ids.clone(), config);
                let reply = msg.reply(Protocol::InitOk, n.get_next_msg_id());
                n.send(reply);
                node = Some(n);
            }
            Protocol::InitOk => {}
            _ => {
                let n = node.as_mut().expect("received message before Init");
                if let Some(reply_to) = msg.body.in_reply_to {
                    n.ack(reply_to);
                }
                handler.handle(msg, n);
            }
        }
    }
}