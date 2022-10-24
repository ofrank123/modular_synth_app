use std::f32::consts::PI;

use crate::{console_log, port_panic, Buffer, Node};

use super::{InputPorts, OutputPorts, ParamValue, PortType, NO_PORT};

enum OscType {
    Sine,
    Square,
}

struct Oscillator {
    sample_rate: f32,
    phase: f32,
    offset_freq: f32,
    base_freq: f32,
    osc_type: OscType,
}

impl Oscillator {
    fn new(sample_rate: f32) -> Self {
        return Oscillator {
            sample_rate,
            phase: 0.0,
            offset_freq: 0.0,
            base_freq: 440.0,
            osc_type: OscType::Sine,
        };
    }

    fn next(&mut self) -> f32 {
        let sample = match self.osc_type {
            OscType::Sine => self.sine_sample(),
            OscType::Square => self.square_sample(),
        };

        // Clip Frequency
        let freq = (self.base_freq + self.offset_freq).max(0.0);

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
        let mut sample = -1.0;
        if self.phase > PI {
            sample = 1.0;
        }
        sample
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

// Convert a CV value to a pitch offset
fn val_to_pitch_offset(val: f32) -> f32 {
    val * 1000.0
}

impl Node for OscNode {
    fn update_param(&mut self, name: &str, param: ParamValue) {
        match (name, param) {
            ("frequency", ParamValue::Num(n)) => self.oscillator.base_freq = n,
            ("type", ParamValue::Str(s)) => match s.as_str() {
                "sine" => self.oscillator.osc_type = OscType::Sine,
                "square" => self.oscillator.osc_type = OscType::Square,
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
            (PortType::In, "Frequency") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        let freq_in = match inputs.get(&0) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        for buffer in output_bufs {
            for i in 0..Buffer::LEN {
                self.oscillator.offset_freq = val_to_pitch_offset(freq_in[i]);
                let next_sample = self.oscillator.next();
                buffer[i] = next_sample;
            }
        }
    }
}
