use std::f32::consts::PI;

use crate::{console_log, port_panic, Buffer, Node};

use super::{InputPorts, OutputPorts, ParamValue, PortType, NO_PORT};

enum OscType {
    Sine,
    Square,
    Saw,
    Triangle,
}

struct Oscillator {
    sample_rate: f32,
    phase: f32,
    base_freq: f32,
    coarse_freq: f32,
    coarse_freq_offset: f32,
    fine_freq: f32,
    fine_freq_offset: f32,
    osc_type: OscType,
}

fn freq_from_midi(midi_val: f32) -> f32 {
    440.0 * 2.0_f32.powf((midi_val - 69.0) / 12.0)
}

impl Oscillator {
    fn new(sample_rate: f32) -> Self {
        return Oscillator {
            sample_rate,
            phase: 0.0,
            base_freq: 69.0,
            coarse_freq: 0.0,
            coarse_freq_offset: 0.0,
            fine_freq: 0.0,
            fine_freq_offset: 0.0,
            osc_type: OscType::Sine,
        };
    }

    fn get_freq(&mut self) -> f32 {
        let coarse = (self.coarse_freq + self.coarse_freq_offset).clamp(-12.0, 12.0);
        let fine = (self.fine_freq + self.fine_freq_offset).clamp(-100.0, 100.0);

        freq_from_midi(self.base_freq + coarse + (fine / 100.0))
    }

    fn next(&mut self) -> f32 {
        let sample = match self.osc_type {
            OscType::Sine => self.sine_sample(),
            OscType::Square => self.square_sample(),
            OscType::Saw => self.saw_sample(),
            OscType::Triangle => self.tri_sample(),
        };

        // Clip Frequency
        let freq = self.get_freq().max(0.0);

        self.phase += (2.0 * PI * freq) / self.sample_rate;

        if self.phase > 4.0 * PI || self.phase < 0.0 {
            console_log!("{}", self.phase);
        }

        self.phase = self.phase.rem_euclid(2.0 * PI);

        sample
    }

    fn sine_sample(&self) -> f32 {
        self.phase.sin()
    }
    fn square_sample(&self) -> f32 {
        if self.phase > PI {
            1.0
        } else {
            -1.0
        }
    }
    fn saw_sample(&self) -> f32 {
        ((self.phase / PI) - 1.0) * -1.0
    }
    fn tri_sample(&self) -> f32 {
        if self.phase < PI {
            (2.0 * self.phase / PI) - 1.0
        } else {
            ((-2.0 * self.phase) / PI) + 3.0
        }
    }
}

pub struct OscNode {
    oscillator: Oscillator,
}

impl OscNode {
    const OUT_PORTS: [u32; 1] = [0];
    pub fn new(sample_rate: f64) -> Self {
        OscNode {
            oscillator: Oscillator::new(sample_rate as f32),
        }
    }
}

impl Node for OscNode {
    fn update_param(&mut self, name: &str, param: ParamValue) {
        match (name, param) {
            ("base_pitch", ParamValue::Num(n)) => self.oscillator.base_freq = n,
            ("coarse_pitch", ParamValue::Num(n)) => self.oscillator.coarse_freq = n,
            ("fine_pitch", ParamValue::Num(n)) => self.oscillator.fine_freq = n,
            ("type", ParamValue::Str(s)) => match s.as_str() {
                "sine" => self.oscillator.osc_type = OscType::Sine,
                "square" => self.oscillator.osc_type = OscType::Square,
                "saw" => self.oscillator.osc_type = OscType::Saw,
                "tri" => self.oscillator.osc_type = OscType::Triangle,
                _ => panic!("Invalid osc type"),
            },
            (_, _) => panic!("Invalid param update on oscillator"),
        }
    }

    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_port(&self, name: &str, port_type: super::PortType) -> u32 {
        match (port_type, name) {
            (PortType::Out, "Audio") => 0,
            (PortType::In, "Coarse Pitch") => 0,
            (PortType::In, "Fine Pitch") => 1,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        let coarse_in = match inputs.get(&0) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let fine_in = match inputs.get(&1) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        for buffer in output_bufs {
            for i in 0..Buffer::LEN {
                self.oscillator.coarse_freq_offset = coarse_in[i] * 12.0;
                self.oscillator.fine_freq_offset = fine_in[i] * 100.0;
                let next_sample = self.oscillator.next();
                buffer[i] = next_sample;
            }
        }
    }
}
