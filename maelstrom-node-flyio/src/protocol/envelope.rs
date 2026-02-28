use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Envelope<B> {
    src: String,
    dest: String,
    body: B
}