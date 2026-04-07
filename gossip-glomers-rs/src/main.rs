fn main() {}

use std::io::BufRead;

use gossip_glomers_rs::io::{Message, parse};
use gossip_glomers_rs::node::Node;
use gossip_glomers_rs::protocol::Protocol;

pub trait Handler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node);
}

fn run<H: Handler>(mut handler: H) {
    let stdin = std::io::stdin();
    let mut node: Option<Node> = None;

    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let msg: Message<Protocol> = parse(&line);

        match &msg.body.contents {
            Protocol::Init { node_id, node_ids } => {
                let n = Node::new(node_id.clone(), node_ids.clone());
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

fn send(msg: &Message<Protocol>) {
    let out = std::io::stdout().lock();
    serde_json::to_writer(out, msg).unwrap();
    println!();
}


