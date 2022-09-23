use wasm_bindgen::prelude::*;

use crate::buffer::OutputBuffer;
use crate::nodes::{OutputNode, SquareNode};
use dasp::graph::{BoxedNode, NodeData};
use petgraph::graph::NodeIndex;
use petgraph::{self as petgraph};
use std::cell::RefCell;
use std::rc::Rc;

type Graph = petgraph::graph::DiGraph<NodeData<BoxedNode>, (), u32>;
type Processor = dasp::graph::Processor<Graph>;

#[wasm_bindgen]
pub struct AudioManager {
    output_buffer: Rc<RefCell<OutputBuffer>>,
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
        let output_buffer = Rc::new(RefCell::new(OutputBuffer::new()));

        let square_node = SquareNode::new(sample_rate);
        let square_node_idx = graph.add_node(NodeData::new1(BoxedNode::new(square_node)));

        let output_node = OutputNode::new(output_buffer.clone());
        let output_node_idx = graph.add_node(NodeData::new1(BoxedNode::new(output_node)));

        graph.add_edge(square_node_idx, output_node_idx, ());

        AudioManager {
            output_buffer: output_buffer.clone(),
            output_node_idx,
            sample_rate,
            graph,
            processor,
        }
    }

    fn get_samples_to_read(&self) -> usize {
        let output_buffer = self.output_buffer.borrow();
        output_buffer.samples_to_read()
    }

    pub fn get_samples(&mut self, n_samples: usize) -> Vec<f32> {
        let mut samples = Vec::new();
        let mut samples_to_read = self.get_samples_to_read();

        while samples_to_read < n_samples {
            self.processor
                .process(&mut self.graph, self.output_node_idx);
            samples_to_read = self.get_samples_to_read();
        }

        {
            let mut output_buffer_mut = self.output_buffer.borrow_mut();
            for _ in 0..n_samples {
                samples.push(output_buffer_mut.read());
            }
        }

        samples
    }
}
