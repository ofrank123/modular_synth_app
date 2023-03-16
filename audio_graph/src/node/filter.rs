use crate::{port_panic, Buffer, Node};

use super::{ParamValue, PortType, NO_PORT};

const BUFF_SIZE: usize = 100;

pub struct FilterNode {
    ring_buffer: [f32; BUFF_SIZE],
    buff_idx: usize,
    falloff: f32,
}

impl FilterNode {
    const OUT_PORTS: [u32; 1] = [0];
    const IN_PORTS: [u32; 1] = [0];

    pub fn new() -> Self {
        FilterNode {
            ring_buffer: [0.0; BUFF_SIZE],
            buff_idx: 0,
            falloff: 0.5,
        }
    }
}

impl Node for FilterNode {
    fn update_param(&mut self, name: &str, param: super::ParamValue) {
        match (name, param) {
            ("falloff", ParamValue::Num(n)) => {
                self.falloff = n;
            }
            (_, _) => panic!("Invalid param updated on a Filter"),
        }
    }

    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_input_ports(&self) -> &[u32] {
        &Self::IN_PORTS
    }

    fn get_port(&self, name: &str, port_type: super::PortType) -> u32 {
        match (port_type, name) {
            (PortType::Out, "Audio") => 0,
            (PortType::In, "Audio") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &super::InputPorts, output: &mut super::OutputPorts) {
        let audio_in = match inputs.get(&0) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        let max_sum: f32 = (0..self.ring_buffer.len())
            .map(|n| self.falloff.powf(n as f32))
            .sum();

        for buffer in output_bufs {
            for i in 0..Buffer::LEN {
                self.ring_buffer[self.buff_idx] = audio_in[i];
                buffer[i] = (0..self.ring_buffer.len())
                    .map(|n| {
                        self.falloff.powf(n as f32)
                            * self.ring_buffer[(self.buff_idx + self.ring_buffer.len() - n)
                                % self.ring_buffer.len()]
                    })
                    .sum::<f32>()
                    / max_sum;

                self.ring_buffer[self.buff_idx] = buffer[i];

                self.buff_idx = (self.buff_idx + 1) % self.ring_buffer.len();
            }
        }
    }
}
