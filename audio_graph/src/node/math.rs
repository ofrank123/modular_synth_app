use crate::{port_panic, Buffer, Node};

use super::{ParamValue, PortType, NO_PORT};

#[derive(Debug)]
pub struct MathInput<'a> {
    buffers: &'a [Buffer],
    av_buffers: &'a [Buffer],
    connected: bool,
}

pub struct MathNode {
    p_attenuverters: [f32; 4],
    attenuverters: [f32; 4],
}

impl MathNode {
    const OUT_PORTS: [u32; 5] = [0, 1, 2, 3, 4];
    const IN_PORTS: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    pub fn new() -> Self {
        MathNode {
            p_attenuverters: [0.0, 0.0, 0.0, 0.0],
            attenuverters: [0.0, 0.0, 0.0, 0.0],
        }
    }

    fn set_attenuverter(&mut self, channel: usize, value: f32) {
        self.attenuverters[channel] = value;
    }

    // Update the p_attenuverter array
    fn update_prev_av(&mut self) {
        for i in 0..self.attenuverters.len() {
            self.p_attenuverters[i] = self.attenuverters[i];
        }
    }

    fn get_av_value(&self, channel: usize, sample: usize) -> f32 {
        let curr = self.attenuverters[channel];
        let prev = self.p_attenuverters[channel];

        prev + sample as f32 * ((curr - prev) / Buffer::LEN as f32)
    }
}

impl Node for MathNode {
    fn update_param(&mut self, name: &str, param: super::ParamValue) {
        match (name, param) {
            ("attenuverter1", ParamValue::Num(n)) => self.set_attenuverter(0, n),
            ("attenuverter2", ParamValue::Num(n)) => self.set_attenuverter(1, n),
            ("attenuverter3", ParamValue::Num(n)) => self.set_attenuverter(2, n),
            ("attenuverter4", ParamValue::Num(n)) => self.set_attenuverter(3, n),
            (_, _) => panic!("Invalid parameter update on Math node"),
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
            (PortType::In, "In 1") => 0,
            (PortType::In, "In 2") => 1,
            (PortType::In, "In 3") => 2,
            (PortType::In, "In 4") => 3,
            (PortType::In, "AV 1") => 4,
            (PortType::In, "AV 2") => 5,
            (PortType::In, "AV 3") => 6,
            (PortType::In, "AV 4") => 7,
            (PortType::Out, "Out 1") => 0,
            (PortType::Out, "Out 2") => 1,
            (PortType::Out, "Out 3") => 2,
            (PortType::Out, "Out 4") => 3,
            (PortType::Out, "Sum") => 4,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &super::InputPorts, output: &mut super::OutputPorts) {
        // Get input buffers
        let input_bufs: [MathInput; 4] =
            [0, 1, 2, 3].map(|n| match (inputs.get(&n), inputs.get(&(n + 4))) {
                (Some(input), Some(av_input)) => MathInput {
                    buffers: input.buffers(),
                    av_buffers: av_input.buffers(),
                    connected: true,
                },
                (_, _) => MathInput {
                    buffers: &[Buffer::SILENT],
                    av_buffers: &[Buffer::SILENT],
                    connected: false,
                },
            });

        // Get the outpus
        let mut outputs: [&mut Vec<Buffer>; 5] =
            output.get_many_mut([&0, &1, &2, &3, &4]).expect(NO_PORT);

        // Split outputs between the attenuverting outputs and the summing output
        let (av_outputs, sum_output) = match outputs[..].split_at_mut(4) {
            (av_outputs, [sum_output]) => (av_outputs, sum_output),
            _ => panic!("{}", NO_PORT), // Panic if there's no summing output
        };

        // Iterate through inputs and attenuverting outputs
        for (n, (av_output, input)) in av_outputs.iter_mut().zip(input_bufs).enumerate() {
            // Ensure summing output has same number of outputs as inputs
            if n == 0 {
                sum_output.append(&mut vec![
                    Buffer::SILENT;
                    0.max(input.buffers.len() - av_output.len())
                ]);

                // Silence the sum buffers
                for chan in sum_output.iter_mut() {
                    chan.silence();
                }
            }
            if input.connected {
                // Add necessary buffers to ouput
                av_output.append(&mut vec![
                    Buffer::SILENT;
                    0.max(input.buffers.len() - av_output.len())
                ]);

                // Loop through channels of the inputs and outputs, which should at this point all be the same number
                for (sum_output, (out_buffer, (in_buffer, av_buffer))) in sum_output.iter_mut().zip(
                    av_output
                        .iter_mut()
                        .zip(input.buffers.iter().zip(input.av_buffers)),
                ) {
                    for i in 0..Buffer::LEN {
                        // Calculate next sample
                        let sample = (self.get_av_value(n, i) + av_buffer[i]).clamp(-1.0, 1.0)
                            * in_buffer[i];
                        // Set attenuverting output
                        out_buffer[i] = sample;
                        // Add to summing output
                        sum_output[i] += sample;
                    }
                }
            } else {
                for buffer in av_output.iter_mut() {
                    for i in 0..Buffer::LEN {
                        buffer[i] = self.attenuverters[n]
                    }
                }
            }
        }

        self.update_prev_av()
    }
}
