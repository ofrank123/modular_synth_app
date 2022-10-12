use wasm_bindgen::prelude::*;

use audio_graph::node::{OscNode, OutputSink, ParamValue};
use audio_graph::Graph;
use audio_graph::NodeData;
use petgraph::graph::NodeIndex;
use petgraph::{self as petgraph};

use crate::messages::{Message, MessageQueue};

type Processor = audio_graph::Processor;

#[wasm_bindgen]
pub struct AudioManager {
    message_queue_out: MessageQueue,
    message_queue_in: MessageQueue,
    output_buffer_ptr: *const f32,
    output_node_idx: NodeIndex,
    graph: Graph,
    processor: Processor,
    sample_rate: f64,
}

#[wasm_bindgen]
impl AudioManager {
    pub fn new(sample_rate: f64) -> Self {
        let mut graph: Graph = Graph::with_capacity(1024, 1024);
        let processor = Processor::with_capacity(1024);
        let buffer = Box::new([0.0; 128]);
        let output_buffer_ptr = buffer.as_ptr();

        let output_node = OutputSink::new(buffer);
        let output_node_idx: u32 = graph.add_node(NodeData::boxed(output_node)).index() as u32;

        let mut am = AudioManager {
            message_queue_out: MessageQueue::new(),
            message_queue_in: MessageQueue::new(),
            output_buffer_ptr,
            output_node_idx: output_node_idx.into(),
            sample_rate,
            graph,
            processor,
        };

        am.send_message(Message::node_created(output_node_idx, "output"));

        let osc_node_idx = am.create_node("oscillator");
        am.connect(osc_node_idx, "Audio", output_node_idx, "Audio");

        am
    }

    pub fn create_node(&mut self, node_type: &str) -> u32 {
        let node_idx = match node_type {
            "oscillator" => {
                let osc_node = OscNode::new(self.sample_rate);
                self.graph.add_node(NodeData::boxed(osc_node)).index() as u32
            }
            t => panic!("No such node type: {}", t),
        };
        self.send_message(Message::node_created(node_idx, node_type));
        node_idx
    }

    pub fn connect(
        &mut self,
        out_node_idx: u32,
        out_node_port: &str,
        in_node_idx: u32,
        in_node_port: &str,
    ) {
        let out_node = (out_node_idx, out_node_port);
        let in_node = (in_node_idx, in_node_port);
        let edge_idx = audio_graph::add_graph_edge(&mut self.graph, out_node, in_node);
        self.send_message(Message::node_connected(edge_idx, out_node, in_node))
    }

    pub fn disconnect(&mut self, edge_idx: u32) {
        audio_graph::remove_graph_edge(&mut self.graph, edge_idx);
        self.send_message(Message::connection_removed(edge_idx));
    }

    pub fn has_message(&self) -> bool {
        self.message_queue_out.has_next()
    }

    pub fn next_message(&mut self) -> Message {
        self.message_queue_out.pop().expect("No messages in queue")
    }

    pub fn add_message(&mut self, message: Message) {
        self.message_queue_in.push(message);
    }

    pub fn get_output_ptr(&self) -> *const f32 {
        self.output_buffer_ptr
    }

    pub fn process(&mut self) {
        if self.message_queue_in.has_next() {
            self.handle_in_messages();
        }

        self.processor
            .process(&mut self.graph, self.output_node_idx);
    }
}

impl AudioManager {
    fn handle_in_messages(&mut self) {
        let drain = self.message_queue_in.drain();
        for mut message in drain {
            let name = message.get_name();
            match name.as_str() {
                "update-node-param" => {
                    let node_idx = message
                        .get_data("id".to_string())
                        .get_str()
                        .parse::<usize>()
                        .expect("Could not parse id");
                    let node = &mut self.graph[NodeIndex::new(node_idx)].node;
                    let name = message.get_data("name".to_string()).get_str();
                    let value = message.get_data("value".to_string());

                    // Ugly ass seam, not sure quite how to do this properly
                    if value.is_float() {
                        node.update_param(name.as_str(), ParamValue::Num(value.get_flt()))
                    } else {
                        node.update_param(name.as_str(), ParamValue::Str(value.get_str()))
                    }
                }
                "remove-connection" => {
                    let edge_idx = message
                        .get_data("id".to_string())
                        .get_str()
                        .parse::<usize>()
                        .expect("Could not parse edge id")
                        as u32;

                    audio_graph::remove_graph_edge(&mut self.graph, edge_idx);

                    // Can't use send_message, because it borrows all of self
                    self.message_queue_out
                        .push(Message::connection_removed(edge_idx))
                }
                "add-connection" => {
                    let in_node_idx = message
                        .get_data("in_node".to_string())
                        .get_str()
                        .parse::<u32>()
                        .expect("Could not parse");

                    let out_node_idx = message
                        .get_data("out_node".to_string())
                        .get_str()
                        .parse::<u32>()
                        .expect("Could not parse");

                    let in_port = message.get_data("in_port".to_string()).get_str();
                    let out_port = message.get_data("out_port".to_string()).get_str();

                    let edge_id = audio_graph::add_graph_edge(
                        &mut self.graph,
                        (out_node_idx, out_port.as_str()),
                        (in_node_idx, in_port.as_str()),
                    );

                    self.message_queue_out.push(Message::node_connected(
                        edge_id,
                        (out_node_idx, out_port.as_str()),
                        (in_node_idx, in_port.as_str()),
                    ))
                }
                "add-module" => {
                    let mod_type = message.get_data("modType".to_string()).get_str();
                    let node_idx = match mod_type.as_str() {
                        "oscillator" => {
                            let osc_node = OscNode::new(self.sample_rate);
                            self.graph.add_node(NodeData::boxed(osc_node)).index() as u32
                        }
                        t => panic!("No such node type: {}", t),
                    };

                    self.message_queue_out
                        .push(Message::node_created(node_idx, mod_type.as_str()));
                }
                _ => panic!("Unsupported message"),
            }
        }
    }

    fn send_message(&mut self, msg: Message) {
        self.message_queue_out.push(msg);
    }
}
