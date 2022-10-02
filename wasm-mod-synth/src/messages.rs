use std::collections::{HashMap, VecDeque};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MessageParamData {
    float: Option<f32>,
    string: Option<String>,
}

impl From<f32> for MessageParamData {
    fn from(f: f32) -> Self {
        MessageParamData {
            float: Some(f),
            string: None,
        }
    }
}

impl From<u32> for MessageParamData {
    fn from(f: u32) -> Self {
        MessageParamData {
            float: Some(f as f32),
            string: None,
        }
    }
}

impl From<&str> for MessageParamData {
    fn from(s: &str) -> Self {
        MessageParamData {
            float: None,
            string: Some(s.to_string()),
        }
    }
}

impl From<String> for MessageParamData {
    fn from(s: String) -> Self {
        MessageParamData {
            float: None,
            string: Some(s),
        }
    }
}

#[wasm_bindgen]
impl MessageParamData {
    pub fn get_str(self) -> String {
        self.string.expect("Not a string!")
    }

    pub fn get_flt(self) -> f32 {
        self.float.expect("Not a float!")
    }
}

#[wasm_bindgen]
pub struct Message {
    name: String,
    data: HashMap<String, MessageParamData>,
}

#[wasm_bindgen]
impl Message {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_data(&mut self, field: String) -> MessageParamData {
        self.data
            .remove(&field)
            .expect("No such field on message object!")
    }
}

impl Message {
    pub fn node_created(node_id: u32, node_type: &str) -> Self {
        Message {
            name: "node_created".into(),
            data: HashMap::from([
                ("node_id".to_string(), node_id.into()),
                ("node_type".to_string(), node_type.into()),
            ]),
        }
    }

    pub fn node_connected(
        (out_node_id, out_node_port): (u32, &str),
        (in_node_id, in_node_port): (u32, &str),
    ) -> Self {
        Message {
            name: "node_connected".into(),
            data: HashMap::from([
                ("out_node_id".to_string(), out_node_id.into()),
                ("out_node_port".to_string(), out_node_port.into()),
                ("in_node_id".to_string(), in_node_id.into()),
                ("in_node_port".to_string(), in_node_port.into()),
            ]),
        }
    }
}

pub struct MessageQueue {
    messages: VecDeque<Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            messages: VecDeque::new(),
        }
    }

    pub fn push(&mut self, msg: Message) {
        self.messages.push_back(msg);
    }

    pub fn pop(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }

    pub fn has_next(&self) -> bool {
        self.messages.len() > 0
    }
}
