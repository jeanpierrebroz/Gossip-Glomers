use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Protocol {
    // setup
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Error {
        code: u16,
        text: String,
    },
    // challenge 1
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    // challenge 2
    Generate,
    GenerateOk {
        id: String,
    },
    // challenge 3
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    BroadcastMany {
        messages: Vec<usize>,
    },
    BroadcastManyOk,
    Read,
    #[serde(rename = "read_ok")]
    ReadOk {
        #[serde(skip_serializing_if = "Option::is_none")]
        messages: Option<Vec<usize>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Value>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    // challenge 4
    Add {
        delta: usize,
    },
    AddOk,
    Write {
        key: String,
        value: Value,
    },
    WriteOk,
    Cas {
        key: String,
        from: Value,
        to: Value,
    },
    CasOk,
    // challenge 5
    Send {
        key: String,
        msg: Value,
    },
    SendOk {
        offset: usize,
    },
    Poll {
        offsets: HashMap<String, usize>,
    },
    PollOk {
        msgs: HashMap<String, Vec<(usize, Value)>>,
    },
    ListCommittedOffsets {
        keys: Vec<String>,
    },
    ListCommittedOffsetsOk {
        offsets: HashMap<String, usize>,
    },
    CommitOffsets {
        offsets: HashMap<String, usize>,
    },
    CommitOffsetsOk,
    Txn {
        txn: Vec<(String, String, Value)>,
    },
    TxnOk {
        txn: Vec<(String, String, Value)>,
    },
}