use crate::{port_panic, Node};

use super::{InputPorts, OutputPorts, ParamValue, PortType};

#[derive(Clone, Debug, PartialEq)]
pub struct DelayNode {
    buffer: Vec<f32>,
    length: usize,   // Length in samples
    length_s: usize, // User defined length (samples)
    length_ms: f32,  // User defined length (ms)
    read_head: usize,
    sample_rate: f64,
}

impl DelayNode {
    const OUT_PORTS: [u32; 1] = [0];

    pub fn resize_buffer(&mut self) {
        self.length =
            (self.length_ms * (self.sample_rate as f32 / 1000.0)) as usize + self.length_s;

        self.buffer.resize(self.length, 0.0);
        self.length = self.length as usize;
        if self.length as usize != 0 {
            self.read_head = self.read_head % self.length as usize;
        }
    }

    pub fn new(sample_rate: f64) -> Self {
        let mut ret = DelayNode {
            buffer: Vec::new(),
            length: 0,
            length_s: 512,
            length_ms: 0.0,
            read_head: 0,
            sample_rate: sample_rate,
        };
        ret.resize_buffer();
        ret
    }
}

impl Node for DelayNode {
    fn update_param(&mut self, name: &str, param: super::ParamValue) {
        match (name, param) {
            ("length_s", ParamValue::Num(n)) => {
                self.length_s = n as usize;
                self.resize_buffer();
            }
            ("length_ms", ParamValue::Num(n)) => {
                self.length_ms = n;
                self.resize_buffer();
            }
            (_, _) => panic!("Invalid param updated on a Delay"),
        }
    }

    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match (port_type, name) {
            (PortType::In, "Audio") => 0,
            (PortType::Out, "Audio") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        // Retrieve the audio input, ignore any others.
        let input = match inputs.get(&0) {
            Some(input) => input,
            None => return,
        };

        // Retrieve output buffer
        let output = match output.get_mut(&0) {
            Some(output) => output,
            None => return,
        };

        if self.length == 0 {
            for (in_sample, out_sample) in input.buffers()[0].iter().zip(output[0].iter_mut()) {
                *out_sample = *in_sample;
            }
        } else {
            for (in_sample, out_sample) in input.buffers()[0].iter().zip(output[0].iter_mut()) {
                *out_sample = self.buffer[self.read_head];
                self.buffer[self.read_head] = *in_sample;
                self.read_head = (self.read_head + 1) % self.length;
            }
        }
    }
}
