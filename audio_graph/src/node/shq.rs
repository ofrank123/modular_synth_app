use crate::{port_panic, Buffer, Node};

use super::{ParamValue, PortType, NO_PORT};

pub struct ShqNode {
    sample_rate: f64,
    hold_time: usize, // Hold time in samples
    samples_passed: usize,
    held_sample: f32,
}

impl ShqNode {
    const OUT_PORTS: [u32; 1] = [0];

    pub fn new(sample_rate: f64) -> Self {
        return ShqNode {
            sample_rate,
            hold_time: 0,
            samples_passed: 1,
            held_sample: 0.0,
        };
    }
}

impl Node for ShqNode {
    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_port(&self, name: &str, port_type: super::PortType) -> u32 {
        match (port_type, name) {
            (PortType::Out, "Out") => 0,
            (PortType::In, "In") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn update_param(&mut self, name: &str, param: super::ParamValue) {
        match (name, param) {
            ("hold_time", ParamValue::Num(n)) => {
                self.hold_time = (n * (self.sample_rate as f32 / 1000.0)).floor() as usize;
            }
            _ => panic!("Invalid param update on Shq"),
        }
    }

    fn process(&mut self, inputs: &super::InputPorts, output: &mut super::OutputPorts) {
        let input_bufs = match inputs.get(&0) {
            Some(n) => n.buffers(),
            None => &[Buffer::SILENT],
        };

        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        for (out_buf, in_buf) in output_bufs.iter_mut().zip(input_bufs) {
            for i in 0..Buffer::LEN {
                // Check if we need to update the sample we're holding
                if self.samples_passed > self.hold_time {
                    self.held_sample = in_buf[i];
                    self.samples_passed = 0;
                }

                // Increment samples passed
                self.samples_passed += 1;
                // Set output buffer
                out_buf[i] = self.held_sample;
            }
        }
    }
}
