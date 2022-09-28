use wasm_bindgen::prelude::*;

use audio_graph::node::{OscNode, OutputSink};
use audio_graph::Graph;
use audio_graph::NodeData;
use petgraph::graph::NodeIndex;
use petgraph::{self as petgraph};

type Processor = audio_graph::Processor;

#[wasm_bindgen]
pub struct AudioManager {
    output_buffer_ptr: *const f32,
    output_node_idx: NodeIndex,
    graph: Graph,
    processor: Processor,
    _sample_rate: f64,
}

#[wasm_bindgen]
impl AudioManager {
    pub fn new(sample_rate: f64) -> Self {
        let mut graph: Graph = Graph::with_capacity(1024, 1024);
        let processor = Processor::with_capacity(1024);
        let buffer = Box::new([0.0; 128]);
        let output_buffer_ptr = buffer.as_ptr();

        let osc_node = OscNode::new(sample_rate);
        let osc_node_idx = graph.add_node(NodeData::boxed(osc_node));

        let output_node = OutputSink::new(buffer);
        let output_node_idx = graph.add_node(NodeData::boxed(output_node));

        audio_graph::add_graph_edge(
            &mut graph,
            (osc_node_idx, "Audio"),
            (output_node_idx, "Audio"),
        );

        AudioManager {
            output_buffer_ptr,
            output_node_idx,
            _sample_rate: sample_rate,
            graph,
            processor,
        }
    }

    pub fn get_output_ptr(&self) -> *const f32 {
        self.output_buffer_ptr
    }

    pub fn process(&mut self) {
        self.processor
            .process(&mut self.graph, self.output_node_idx);
    }
}
