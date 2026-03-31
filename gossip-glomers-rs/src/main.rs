use std::io::{self, BufRead};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;

fn main() {
    read();
}

enum SystemType {
    Init,
    InitOk
}

#[derive(Deserialize)]
enum MessageType<B> {
    Init,
    InitOk,
    
}


fn read() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match &line {
            Ok(s) => {
                println!("Read line: {:?}", s);
            }

            Err(s) => {
                println!("Couldn't read line: {:?}", s);
            }
        }
    }
}
