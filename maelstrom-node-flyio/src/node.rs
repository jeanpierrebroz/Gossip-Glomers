use std::thread;

use crate::protocol::envelope::Envelope;

pub fn run() {
    //handle Init and InitOk messages
    thread::spawn(move || {
                    loop {
                        thread::sleep(Duration::from_millis(500));
                        let _ = outbound_clone.send(Envelope {
                            src: node_id_clone.clone(),
                            dest: node_id_clone.clone(),
                            body: Message::Tick {},
                        });
                    }
                });
}