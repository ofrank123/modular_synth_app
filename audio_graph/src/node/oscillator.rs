use crate::{console_log, port_panic, Buffer, Node};
use dasp::{signal, Signal};

use super::{InputPorts, OutputPorts, PortType, NO_PORT};

pub struct OscNode {
    signal: Box<dyn Signal<Frame = f64>>,
}

impl OscNode {
    const OUT_PORTS: [u32; 1] = [0];

    pub fn new(sample_rate: f64) -> Self {
        let signal = Box::new(
            signal::rate(sample_rate).const_hz(440.0).sine().add_amp(
                signal::rate(sample_rate)
                    .const_hz(880.0)
                    .sine()
                    .scale_amp(0.5),
            ),
        );

        OscNode { signal }
    }
}

impl Node for OscNode {
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
                let next_sample = self.signal.next() as f32;
                buffer[i] = next_sample as f32;
            }
        }
    }
}
