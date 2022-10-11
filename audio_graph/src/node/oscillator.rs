use std::f32::consts::PI;

use crate::{port_panic, Buffer, Node};

use super::{InputPorts, OutputPorts, ParamValue, PortType, NO_PORT};

enum OscType {
    Sine,
    Square,
}

struct Oscillator {
    sample_rate: f32,
    phase: f32,
    freq: f32,
    osc_type: OscType,
}

impl Oscillator {
    fn new(sample_rate: f32) -> Self {
        return Oscillator {
            sample_rate,
            phase: 0.0,
            freq: 440.0,
            osc_type: OscType::Square,
        };
    }

    fn next(&mut self) -> f32 {
        let sample = match self.osc_type {
            OscType::Sine => self.sine_sample(),
            OscType::Square => self.square_sample(),
        };

        // Get new phase
        self.phase += (2.0 * PI * self.freq) / self.sample_rate;

        // Mod by 2pi
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        sample
    }

    fn sine_sample(&self) -> f32 {
        self.phase.sin() / 2.0
    }
    fn square_sample(&self) -> f32 {
        let mut sample = 0.0;
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

impl Node for OscNode {
    fn update_param(&mut self, name: &str, param: ParamValue) {
        match (name, param) {
            ("frequency", ParamValue::Num(n)) => self.oscillator.freq = n,
            (_, _) => panic!("Invalid param update on oscillator"),
        }
    }

    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_port(&self, name: &str, port_type: super::PortType) -> u32 {
        match (port_type, name) {
            (PortType::Out, "Audio") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, _inputs: &InputPorts, output: &mut OutputPorts) {
        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        for buffer in output_bufs {
            for i in 0..Buffer::LEN {
                // Attenuate the sample
                let next_sample = self.oscillator.next() * 0.5;
                buffer[i] = next_sample;
            }
        }
    }
}
