use wasm_bindgen::prelude::*;

use audio_graph::node::{OutputSink, ParamValue};
use audio_graph::{new_mod, Graph, ModParams};
use audio_graph::{BoxedNode, NodeData};
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
    midi_node_idxs: Vec<u32>,
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
        let output_node_idx: u32 = graph
            .add_node(NodeData::new_boxed(BoxedNode::new(output_node)))
            .index() as u32;

        let mut am = AudioManager {
            message_queue_out: MessageQueue::new(),
            message_queue_in: MessageQueue::new(),
            output_buffer_ptr,
            output_node_idx: output_node_idx.into(),
            midi_node_idxs: vec![],
            sample_rate,
            graph,
            processor,
        };

        am.send_message(Message::node_created(output_node_idx, "output"));
        am.send_message(Message::mod_specs());

        am
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
                        .parse::<u32>()
                        .expect("Could not parse edge id");

                    audio_graph::remove_graph_edge(&mut self.graph, edge_idx);

                    // Can't use send_message, because it borrows all of self
                    self.message_queue_out
                        .push(Message::connection_removed(edge_idx))
                }
                "remove-node" => {
                    let node_idx = message
                        .get_data("id".to_string())
                        .get_str()
                        .parse::<u32>()
                        .expect("Could not parse node id");

                    audio_graph::remove_graph_node(&mut self.graph, node_idx);

                    self.message_queue_out.push(Message::node_removed(node_idx));
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

                    let mod_params = ModParams {
                        sample_rate: self.sample_rate,
                    };
                    // Create module
                    let new_node = new_mod(mod_type.as_str(), mod_params);

                    // Add module, get index
                    let node_idx =
                        self.graph.add_node(NodeData::new_boxed(new_node)).index() as u32;

                    if mod_type == "midi" {
                        self.midi_node_idxs.push(node_idx);
                    }

                    self.message_queue_out
                        .push(Message::node_created(node_idx, mod_type.as_str()));
                }
                "midi-message" => {
                    let message_type = message.get_data("messageType".to_string()).get_str();
                    let note = message.get_data("note".to_string()).get_flt() as u32;

                    for idx in &self.midi_node_idxs {
                        let node = &mut self.graph[NodeIndex::new(*idx as usize)].node;
                        match message_type.as_str() {
                            "NOTE_ON" => node.midi_message(true, note),
                            "NOTE_OFF" => node.midi_message(false, note),
                            _ => panic!("Unsupported Midi message"),
                        }
                    }
                }
                _ => panic!("Unsupported message"),
            }
        }
    }

    fn send_message(&mut self, msg: Message) {
        self.message_queue_out.push(msg);
    }
}
