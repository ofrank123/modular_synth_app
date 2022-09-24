use audio_graph::{Buffer, Input, Node};
use dasp::{signal, Signal};

pub struct SquareNode {
    signal: Box<dyn Signal<Frame = f64>>,
}

impl SquareNode {
    pub fn new(sample_rate: f64) -> Self {
        let signal = Box::new(
            signal::rate(sample_rate)
                .const_hz(440.0)
                .sine()
                .mul_amp(signal::gen(|| 0.5)),
        );

        SquareNode { signal }
    }
}

impl Node for SquareNode {
    fn process(&mut self, _inputs: &[Input], output: &mut [Buffer]) {
        for buffer in output {
            for i in 0..Buffer::LEN {
                buffer[i] = self.signal.next() as f32;
            }
        }
    }
}
