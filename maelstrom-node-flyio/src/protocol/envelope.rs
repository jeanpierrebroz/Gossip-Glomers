use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Envelope<Value> {
    src: String,
    dest: String,
    body: Value
}
