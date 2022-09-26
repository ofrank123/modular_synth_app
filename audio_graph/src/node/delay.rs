use crate::{port_panic, Node};
use dasp_ring_buffer as ring_buffer;

use super::{InputPorts, OutputPorts, PortType, NO_PORT};

/// A delay node, where the delay duration for each channel is equal to the length of the inner
/// ring buffer associated with that channel.
///
/// Assumes that there is one input node, and that the number of input buffers, output buffers and
/// ring buffers all match.
#[derive(Clone, Debug, PartialEq)]
pub struct Delay<S> {
    pub buffer: Vec<ring_buffer::Fixed<S>>,
}

impl<S> Delay<S>
where
    S: ring_buffer::SliceMut<Element = f32>,
{
    const OUT_PORTS: [u32; 1] = [0];
}

impl<S> Node for Delay<S>
where
    S: ring_buffer::SliceMut<Element = f32>,
{
    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }
    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match (port_type, name) {
            (PortType::In, "audio") => 0,
            (PortType::Out, "audio") => 0,
            (t, n) => port_panic!(t, n),
        }
    }
    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        // Retrieve the audio input, ignore any others.
        let input = match inputs.get(&0) {
            Some(input) => input,
            None => return,
        };

        // Apply the delay across each channel.
        for ((ring_buf, in_buf), out_buf) in self
            .buffer
            .iter_mut()
            .zip(input.buffers())
            .zip(output.get_mut(&0).expect(NO_PORT).get_mut(0))
        {
            for (i, out) in out_buf.iter_mut().enumerate() {
                *out = ring_buf.push(in_buf[i]);
            }
        }
    }
}
