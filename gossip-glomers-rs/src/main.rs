fn main() {}

use gossip_glomers_rs::io::Handler;
use gossip_glomers_rs::io::{Message, parse};
use gossip_glomers_rs::node::{Node, RpcRetryConfig};
use gossip_glomers_rs::protocol::Protocol;
use std::io::BufRead;

fn run<H: Handler>(mut handler: H, rpc_retry_config: Option<RpcRetryConfig>) {
    let stdin = std::io::stdin();
    let mut node: Option<Node> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let msg: Message<Protocol> = parse(&line);

        //TODO: ACK here

        match &msg.body.contents {
            Protocol::Init { node_id, node_ids } => {
                let config = rpc_retry_config
                    .clone()
                    .unwrap_or(RpcRetryConfig::default());

                let n = Node::new(node_id.clone(), node_ids.clone(), config);

                let id = n.get_next_msg_id();
                let reply = msg.reply(Protocol::InitOk, id);
                n.send(reply);
                node = Some(n);
            }
            Protocol::InitOk => {}
            _ => {
                let n = node.as_ref().expect("received message before Init");
                n.ack(&msg.body.msg_id);
                handler.handle(msg, n);
            }
        }
    }
}


