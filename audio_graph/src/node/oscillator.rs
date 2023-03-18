use std::f32::consts::PI;

use crate::{oscillators::{
    naive_saw_sample, naive_square_sample, naive_tri_sample, poly_blep, sine_sample,
}, port_panic, Buffer, Node, freq_from_midi};

use super::{InputPorts, OutputPorts, ParamValue, PortType, NO_PORT};

enum OscType {
    Sine,
    NSquare,
    Square,
    NSaw,
    Saw,
    Triangle,
}

struct Oscillator {
    sample_rate: f32,
    phase: f32,
    phase_param_offset: f32,
    phase_offset: f32,
    base_freq: f32,
    base_freq_offset: f32,
    coarse_freq: f32,
    coarse_freq_offset: f32,
    fine_freq: f32,
    fine_freq_offset: f32,
    pulse_width: f32,
    pulse_width_offset: f32,
    osc_type: OscType,
}

impl Oscillator {
    fn new(sample_rate: f32) -> Self {
        return Oscillator {
            sample_rate,
            phase: 0.0,
            phase_param_offset: 0.0,
            phase_offset: 0.0,
            base_freq: 0.0,
            base_freq_offset: 0.0,
            coarse_freq: 0.0,
            coarse_freq_offset: 0.0,
            fine_freq: 0.0,
            fine_freq_offset: 0.0,
            pulse_width: 0.5,
            pulse_width_offset: 0.0,
            osc_type: OscType::Sine,
        };
    }

    fn get_freq(&mut self) -> f32 {
        let base_midi = ((self.base_freq + self.base_freq_offset).clamp(-1.0, 1.0) + 1.0) * 64.0;
        let coarse = (self.coarse_freq + self.coarse_freq_offset).clamp(-12.0, 12.0);
        let fine = (self.fine_freq + self.fine_freq_offset).clamp(-100.0, 100.0);

        freq_from_midi(base_midi + coarse + (fine / 100.0))
    }

    fn next(&mut self) -> f32 {
        // Clip Frequency
        let freq = self.get_freq().max(0.0);

        let phase_inc = (2.0 * PI * freq) / self.sample_rate;

        self.phase = (self.phase + phase_inc).rem_euclid(2.0 * PI);

        let offset_phase =
            (self.phase + self.phase_param_offset + self.phase_offset).rem_euclid(2.0 * PI);
        let t = offset_phase / (2.0 * PI);

        let sample = match self.osc_type {
            OscType::Sine => sine_sample(offset_phase),
            OscType::NSaw => {
                let s = naive_saw_sample(offset_phase);
                s
            }
            OscType::Saw => {
                let s = naive_saw_sample(offset_phase);
                s - poly_blep(phase_inc, t)
            }
            OscType::NSquare => {
                let s =
                    naive_square_sample(offset_phase, self.pulse_width + self.pulse_width_offset);
                s
            }
            OscType::Square => {
                let pw = (self.pulse_width + self.pulse_width_offset).clamp(0.0, 1.0);
                let mut s = naive_square_sample(offset_phase, pw);
                s += poly_blep(phase_inc, t);
                s -= poly_blep(phase_inc, (t + -pw).rem_euclid(1.0));
                s
            }
            OscType::Triangle => naive_tri_sample(offset_phase),
        };

        sample
    }
}

pub struct OscNode {
    oscillator: Oscillator,
}

impl OscNode {
    const OUT_PORTS: [u32; 1] = [0];
    const IN_PORTS: [u32; 5] = [0, 1, 2, 3, 4];

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
            ("phase", ParamValue::Num(n)) => self.oscillator.phase_param_offset = n * 2.0 * PI,
            ("pulse_width", ParamValue::Num(n)) => self.oscillator.pulse_width = n,
            ("type", ParamValue::Str(s)) => match s.as_str() {
                "sine" => self.oscillator.osc_type = OscType::Sine,
                "nsquare" => self.oscillator.osc_type = OscType::NSquare,
                "square" => self.oscillator.osc_type = OscType::Square,
                "saw" => self.oscillator.osc_type = OscType::Saw,
                "nsaw" => self.oscillator.osc_type = OscType::NSaw,
                "tri" => self.oscillator.osc_type = OscType::Triangle,
                _ => panic!("Invalid osc type"),
            },
            (_, _) => panic!("Invalid param update on oscillator"),
        }
    }

    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_input_ports(&self) -> &[u32] {
        &Self::IN_PORTS
    }

    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match (port_type, name) {
            (PortType::Out, "Audio") => 0,
            (PortType::In, "Base Pitch") => 4,
            (PortType::In, "Coarse Pitch") => 0,
            (PortType::In, "Fine Pitch") => 1,
            (PortType::In, "Phase") => 2,
            (PortType::In, "Pulse Width") => 3,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        let base_in = match inputs.get(&4) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let coarse_in = match inputs.get(&0) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let fine_in = match inputs.get(&1) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let phase_in = match inputs.get(&2) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let pulse_in = match inputs.get(&3) {
            Some(n) => &n.buffers()[0],
            None => &Buffer::SILENT,
        };

        let output_bufs = output.get_mut(&0).expect(NO_PORT);

        for buffer in output_bufs {
            for i in 0..Buffer::LEN {
                self.oscillator.base_freq_offset = base_in[i];
                self.oscillator.coarse_freq_offset = coarse_in[i] * 12.0;
                self.oscillator.fine_freq_offset = fine_in[i] * 100.0;
                self.oscillator.phase_offset = phase_in[i] * 2.0 * PI;
                self.oscillator.pulse_width_offset = pulse_in[i];
                let next_sample = self.oscillator.next();
                buffer[i] = next_sample;
            }
        }
    }
}
