use crate::{port_panic, Buffer, Node};

use super::{ParamValue, PortType, NO_PORT};

#[derive(Clone, Debug, PartialEq)]
pub struct EnvelopeNode {
    attack_ms: f32,
    decay_ms: f32,
    release_ms: f32,
    sustain: f32,
    current_note_length_s: usize,

    note_held: bool,
    sample_rate: f64,
}

impl EnvelopeNode {
    const OUT_PORTS: [u32; 1] = [0];
    const IN_PORTS: [u32; 2] = [0, 1];

    pub fn new(sample_rate: f64) -> Self {
        EnvelopeNode {
            attack_ms: 0.0,
            decay_ms: 0.0,
            release_ms: 0.0,
            sustain: 0.5,
            current_note_length_s: 0,
            note_held: false,
            sample_rate,
        }
    }
}

impl Node for EnvelopeNode {
    fn update_param(&mut self, name: &str, param: super::ParamValue) {
        match (name, param) {
            ("attack", ParamValue::Num(n)) => self.attack_ms = n,
            ("decay", ParamValue::Num(n)) => self.decay_ms = n,
            ("sustain", ParamValue::Num(n)) => self.sustain = n,
            ("release", ParamValue::Num(n)) => self.release_ms = n,
            (_, _) => panic!("Invalid param updated on an Envelope"),
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
            (PortType::In, "Gate") => 0,
            (PortType::In, "Audio") => 1,
            (PortType::Out, "Audio") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &super::InputPorts, output: &mut super::OutputPorts) {
        let gate_in = match inputs.get(&0) {
            Some(input) => &input.buffers()[0],
            None => return,
        };

        let audio_in = match inputs.get(&1) {
            Some(input) => &input.buffers()[0],
            None => return,
        };

        let ms_per_samp: f32 = 1000.0 / self.sample_rate as f32;

        for buffer in output.get_mut(&0).expect(NO_PORT) {
            for i in 0..Buffer::LEN {
                self.current_note_length_s += 1;

                if !self.note_held && gate_in[i] > f32::EPSILON {
                    self.note_held = true;
                    self.current_note_length_s = 0;
                } else if self.note_held && gate_in[i] < f32::EPSILON {
                    self.note_held = false;
                    self.current_note_length_s = 0;
                }

                let note_length_ms = self.current_note_length_s as f32 * ms_per_samp;

                if self.note_held {
                    if note_length_ms < self.attack_ms {
                        // ATTACK
                        buffer[i] = audio_in[i] * (note_length_ms / self.attack_ms);
                    } else if note_length_ms < self.attack_ms + self.decay_ms {
                        // DECAY
                        let decay_amt = (note_length_ms - self.attack_ms) / self.decay_ms;
                        buffer[i] = audio_in[i] * (1.0 - (decay_amt * (1.0 - self.sustain)));
                    } else {
                        // SUSTAIN
                        buffer[i] = audio_in[i] * self.sustain;
                    }
                } else {
                    if note_length_ms < self.release_ms {
                        buffer[i] =
                            audio_in[i] * self.sustain * (1.0 - (note_length_ms / self.release_ms));
                    } else {
                        buffer[i] = 0.0;
                    }
                }
            }
        }
    }
}
