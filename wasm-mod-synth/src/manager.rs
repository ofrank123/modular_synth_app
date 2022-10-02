use wasm_bindgen::prelude::*;

use audio_graph::node::{OscNode, OutputSink};
use audio_graph::Graph;
use audio_graph::NodeData;
use petgraph::graph::NodeIndex;
use petgraph::{self as petgraph};

use crate::messages::{Message, MessageQueue};

type Processor = audio_graph::Processor;

#[wasm_bindgen]
pub struct AudioManager {
    message_queue: MessageQueue,
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
            message_queue: MessageQueue::new(),
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
        audio_graph::add_graph_edge(&mut self.graph, out_node, in_node);
        self.send_message(Message::node_connected(out_node, in_node))
    }

    pub fn has_message(&self) -> bool {
        self.message_queue.has_next()
    }

    pub fn next_message(&mut self) -> Message {
        self.message_queue.pop().expect("No messages in queue")
    }

    pub fn get_output_ptr(&self) -> *const f32 {
        self.output_buffer_ptr
    }

    pub fn process(&mut self) {
        self.processor
            .process(&mut self.graph, self.output_node_idx);
    }
}

impl AudioManager {
    fn send_message(&mut self, msg: Message) {
        self.message_queue.push(msg);
    }
}
