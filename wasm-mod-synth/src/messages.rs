use std::{
    collections::{vec_deque::Drain, HashMap, VecDeque},
    fmt,
};

use audio_graph::get_serialized_specs;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct MessageParamData {
    float: Option<f32>,
    string: Option<String>,
}

impl fmt::Display for MessageParamData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self.float {
            Some(flt) => write!(f, "{}", flt),
            None => match &self.string {
                Some(s) => write!(f, "{}", s),
                None => write!(f, "No value!"),
            },
        }
    }
}

#[wasm_bindgen]
impl MessageParamData {
    pub fn float(f: f32) -> Self {
        MessageParamData {
            float: Some(f),
            string: None,
        }
    }

    pub fn string(s: String) -> Self {
        MessageParamData {
            float: None,
            string: Some(s),
        }
    }
}

impl MessageParamData {
    pub fn is_float(&self) -> bool {
        match self.float {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self.string {
            Some(_) => true,
            None => false,
        }
    }
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
    pub fn new(name: String) -> Self {
        Message {
            name,
            data: HashMap::new(),
        }
    }

    pub fn add_param(&mut self, name: String, param: MessageParamData) {
        self.data.insert(name, param);
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_data(&mut self, field: String) -> MessageParamData {
        self.data
            .remove(&field)
            .expect(format!("No such field '{}' on message object", field).as_str())
    }
}

impl Message {
    pub fn mod_specs() -> Self {
        Message {
            name: "mod_specs".into(),
            data: HashMap::from([("modules".to_string(), get_serialized_specs().into())]),
        }
    }

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
        edge_id: u32,
        (out_node_id, out_node_port): (u32, &str),
        (in_node_id, in_node_port): (u32, &str),
    ) -> Self {
        Message {
            name: "node_connected".into(),
            data: HashMap::from([
                ("edge_id".to_string(), edge_id.into()),
                ("out_node_id".to_string(), out_node_id.into()),
                ("out_node_port".to_string(), out_node_port.into()),
                ("in_node_id".to_string(), in_node_id.into()),
                ("in_node_port".to_string(), in_node_port.into()),
            ]),
        }
    }

    pub fn node_removed(node_id: u32) -> Self {
        Message {
            name: "node_removed".into(),
            data: HashMap::from([("node_id".to_string(), node_id.into())]),
        }
    }

    pub fn connection_removed(edge_id: u32) -> Self {
        Message {
            name: "connection_removed".into(),
            data: HashMap::from([("edge_id".to_string(), edge_id.into())]),
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

    pub fn drain(&mut self) -> Drain<'_, Message> {
        self.messages.drain(0..)
    }
}
