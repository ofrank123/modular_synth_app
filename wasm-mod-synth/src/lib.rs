mod utils;
mod buffer;

use buffer::OutputBuffer;
use dasp::signal::{self as signal, Signal};
use dasp::graph::{Node, Buffer, Input, NodeData, BoxedNode};
use petgraph;
use std::f64::consts::PI;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


type Graph = petgraph::graph::DiGraph<NodeData<BoxedNode>, (), u32>;

type Processor = dasp::graph::Processor<Graph>;

pub struct OutNode {
    output_buffer: OutputBuffer,
}

impl Node for OutNode {
    fn process(&mut self, inputs: &[Input], _output: &mut [Buffer]) {
        // Write first channel of first input to output buffer
        let input = &inputs[0];
        let in_buffers = input.buffers();
        let buffer = &in_buffers[0];
        self.output_buffer.write(buffer.into_iter());
    }
}

#[wasm_bindgen]
pub struct AudioManager {
    signal: Box<dyn Signal<Frame = f64>>,
    graph: Graph,
    processor: Processor,
    sample_rate: f64
}

#[wasm_bindgen]
impl AudioManager {
    pub fn new(sample_rate: f64) -> Self {
        let signal = signal::rate(sample_rate).const_hz(440.0).square().mul_amp(signal::gen(|| 0.5));
        let mut graph = Graph::with_capacity(1024, 1024);
        let mut processor = Processor::with_capacity(1024);

        AudioManager { signal: Box::new(signal), sample_rate, graph, processor }
    }

    pub fn get_samples(&mut self, n_samples: usize) -> Vec<f64> {
        let mut samples = Vec::new();

        for _ in 0..n_samples {
            samples.push(self.signal.next());
        }

        samples
    }
}

#[wasm_bindgen]
pub fn get_sine_wave(time: f64, freq: f64) -> f64 {
    0.5 * (freq * PI * 2.0 * time).sin()
}
