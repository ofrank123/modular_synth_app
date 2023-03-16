use crate::{port_panic, Node};

use super::{Buffer, InputPorts, OutputPorts, PortType};

pub struct OutputSink {
    output_buffer: Box<[f32]>,
}

impl OutputSink {
    const OUT_PORTS: [u32; 1] = [0];
    const IN_PORTS: [u32; 1] = [0];

    pub fn new(output_buffer: Box<[f32]>) -> Self {
        OutputSink {
            output_buffer: output_buffer,
        }
    }
}

impl Node for OutputSink {
    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }
    fn get_input_ports(&self) -> &[u32] {
        &Self::IN_PORTS
    }

    fn get_port(&self, name: &str, port_type: super::PortType) -> u32 {
        match (port_type, name) {
            (PortType::In, "Audio") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, _output: &mut OutputPorts) {
        let input = match inputs.get(&0) {
            Some(n) => n,
            None => {
                self.output_buffer.copy_from_slice(&Buffer::SILENT);
                return;
            }
        };
        // Write first channel of first input to output buffer
        let in_buffers = input.buffers();
        let buffer = &in_buffers[0];
        self.output_buffer.copy_from_slice(buffer);
    }
}
