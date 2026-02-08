use std::io::{self, BufRead};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    #[serde(rename = "type")]
    msg_type: String,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    extra: serde_json::Value,
}

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input: Message = serde_json::from_str(&line?)?;
        let body: Payload = serde_json::from_value(input.body)?;

        let reply_type = match body.msg_type.as_str() {
            "init" => "init_ok",
            "echo" => "echo_ok",
            _ => continue,
        };

        let reply_body = Payload {
            msg_type: reply_type.to_string(),
            in_reply_to: body.msg_id,
            msg_id: Some(0), 
            extra: body.extra,
        };

        let reply = Message {
            src: input.dest,
            dest: input.src,
            body: serde_json::to_value(reply_body)?,
        };

        println!("{}", serde_json::to_string(&reply)?);
    }
    Ok(())
}