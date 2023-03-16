use std::f32::consts::PI;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::oscillators::{naive_saw_sample, naive_square_sample, naive_tri_sample, sine_sample};
use crate::{console_log, port_panic, Buffer, Node};

use super::{InputPorts, OutputPorts, ParamValue, PortType, NO_PORT};

enum LFOType {
    Sine,
    Square,
    Saw,
    Triangle,
    Noise,
}

struct Lfo {
    sample_rate: f32,
    phase: f32,
    base_freq: f32,
    freq_offset: f32,
    lfo_type: LFOType,
    pulse_width: f32,
    pulse_width_offset: f32,
    rng: StdRng,
}

impl Lfo {
    fn new(sample_rate: f32) -> Self {
        return Lfo {
            sample_rate,
            phase: 0.0,
            base_freq: 10.0,
            freq_offset: 0.0,
            lfo_type: LFOType::Sine,
            pulse_width: 0.5,
            pulse_width_offset: 0.0,
            rng: StdRng::seed_from_u64(0),
        };
    }

    fn get_freq(&mut self) -> f32 {
        self.base_freq + self.freq_offset
    }

    fn next(&mut self) -> f32 {
        let sample = match self.lfo_type {
            LFOType::Sine => sine_sample(self.phase),
            LFOType::Square => naive_square_sample(
                self.phase,
                (self.pulse_width + self.pulse_width_offset).clamp(0.0, 1.0),
            ),
            LFOType::Saw => naive_saw_sample(self.phase),
            LFOType::Triangle => naive_tri_sample(self.phase),
            LFOType::Noise => {
                (((self.rng.next_u32() as f64 / std::u32::MAX as f64) * 2.0) - 1.0) as f32
            }
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
}

pub struct LfoNode {
    lfo: Lfo,
}

impl LfoNode {
    const OUT_PORTS: [u32; 1] = [0];
    const IN_PORTS: [u32; 2] = [0, 1];
    pub fn new(sample_rate: f64) -> Self {
        LfoNode {
            lfo: Lfo::new(sample_rate as f32),
        }
    }
}

impl Node for LfoNode {
    fn update_param(&mut self, name: &str, param: ParamValue) {
        match (name, param) {
            ("base_pitch", ParamValue::Num(n)) => self.lfo.base_freq = n,
            ("pulse_width", ParamValue::Num(n)) => self.lfo.pulse_width = n,
            ("type", ParamValue::Str(s)) => match s.as_str() {
                "sine" => self.lfo.lfo_type = LFOType::Sine,
                "square" => self.lfo.lfo_type = LFOType::Square,
                "saw" => self.lfo.lfo_type = LFOType::Saw,
                "tri" => self.lfo.lfo_type = LFOType::Triangle,
                "noise" => self.lfo.lfo_type = LFOType::Noise,
                _ => panic!("Invalid lfo type"),
            },
            (_, _) => panic!("Invalid param update on an LFO"),
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
            (PortType::In, "Frequency") => 0,
            (PortType::In, "Pulse Width") => 1,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        let freq_in = match inputs.get(&0) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let pulse_in = match inputs.get(&1) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        for buffer in output_bufs {
            for i in 0..Buffer::LEN {
                self.lfo.pulse_width_offset = pulse_in[i];
                self.lfo.freq_offset = freq_in[i] * 100.0;
                let next_sample = self.lfo.next();
                buffer[i] = next_sample;
            }
        }
    }
}
