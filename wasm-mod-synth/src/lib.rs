mod buffer;
mod utils;

use buffer::OutputBuffer;
use dasp::graph::{BoxedNode, Buffer, Input, Node, NodeData};
use dasp::signal::{self as signal, Signal};
use petgraph::graph::NodeIndex;
use petgraph::{self as petgraph};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

type Graph = petgraph::graph::DiGraph<NodeData<BoxedNode>, (), u32>;

type Processor = dasp::graph::Processor<Graph>;

struct OutputNode {
    output_buffer: Rc<RefCell<OutputBuffer>>,
}

impl OutputNode {
    fn new(output_buffer: Rc<RefCell<OutputBuffer>>) -> Self {
        OutputNode {
            output_buffer: output_buffer,
        }
    }
}

impl Node for OutputNode {
    fn process(&mut self, inputs: &[Input], _output: &mut [Buffer]) {
        // Write first channel of first input to output buffer
        let mut output_buffer = self.output_buffer.borrow_mut();
        let input = &inputs[0];
        let in_buffers = input.buffers();
        let buffer = &in_buffers[0];
        output_buffer.write(buffer.into_iter());
    }
}

struct SquareNode {
    signal: Box<dyn Signal<Frame = f64>>,
}

impl SquareNode {
    fn new(sample_rate: f64) -> Self {
        let signal = Box::new(
            signal::rate(sample_rate)
                .const_hz(440.0)
                .square()
                .mul_amp(signal::gen(|| 0.5)),
        );

        SquareNode { signal }
    }
}

impl Node for SquareNode {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for buffer in output {
            for i in 0..Buffer::LEN {
                buffer[i] = self.signal.next() as f32;
            }
        }
    }
}

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

    pub fn get_samples(&mut self, n_samples: usize) -> Vec<f32> {
        let mut samples = Vec::new();

        {
            let output_buffer = self.output_buffer.borrow();

            while output_buffer.samples_to_read() < n_samples {
                self.processor
                    .process(&mut self.graph, self.output_node_idx);
            }
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

#[wasm_bindgen]
pub fn get_sine_wave(time: f64, freq: f64) -> f64 {
    0.5 * (freq * PI * 2.0 * time).sin()
}
