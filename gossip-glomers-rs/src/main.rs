fn main() {}

use std::io::BufRead;

use gossip_glomers_rs::io::{Message, parse, send};
use gossip_glomers_rs::node::{Node, RpcRetryConfig};
use gossip_glomers_rs::protocol::Protocol;

pub trait Handler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node);
}

fn run<H: Handler>(mut handler: H, rpc_retry_config: Option<RpcRetryConfig>) {
    let stdin = std::io::stdin();
    let mut node: Option<Node> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let msg: Message<Protocol> = parse(&line);

        match &msg.body.contents {
            Protocol::Init { node_id, node_ids } => {
                
                let config = rpc_retry_config.clone().unwrap_or(RpcRetryConfig::default());
                
                let n = Node::new(node_id.clone(), node_ids.clone(), config);
                
                let id = n.get_next_msg_id();
                let reply = msg.reply(Protocol::InitOk, id);
                send(&reply);
                node = Some(n);
            }
            Protocol::InitOk => {} // ignore
            _ => {
                let n = node.as_ref().expect("received message before Init");
                handler.handle(msg, n);
            }
        }
    }
}
