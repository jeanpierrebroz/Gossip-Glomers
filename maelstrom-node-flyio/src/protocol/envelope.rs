use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Envelope<B> {
    pub(crate)src: String,
    pub(crate)dest: String,
    pub(crate)body: B
}
