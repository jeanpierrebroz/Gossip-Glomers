fn main () {
    
}

use std::io::BufRead;

use gossip_glomers_rs::io::{Message, parse};
use gossip_glomers_rs::protocol::Protocol;
use gossip_glomers_rs::node::Node;

pub trait Handler {
    fn handle(&mut self, msg: Message<Protocol>, node: &Node) -> Option<Protocol>;
}


fn run<H: Handler>(mut handler: H) {
    let stdin = std::io::stdin();
    let mut node: Option<Node> = None;
    
    for line in stdin.lock().lines() {
        let line = line.expect("failed to read line");
        let msg: Message<Protocol> = parse(&line);
        
        match msg.body.contents {
                    Protocol::Init { node_id, node_ids } => {
                        node = Some(Node::new(node_id, node_ids));
                        // send InitOk back
                        let reply = msg.reply(Protocol::InitOk);
                        send(&reply);
                    }
                    Protocol::InitOk => {} // ignore
                    _ => {
                        let n = node.as_ref().expect("received message before Init");
                        if let Some(response) = handler.handle(msg, n) {
                            // send response
                        }
                    }
        }
    }
    
}