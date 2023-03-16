use crate::{port_panic, Node};

use super::{InputPorts, OutputPorts, PortType, NO_PORT};

/// A simple node that passes an input directly to the output.
///
/// Works by mem-copying each buffer of the first input to each buffer of the output respectively.
///
/// This can be useful as an intermediary node when feeding the output of a node back into one of
/// its inputs. It can also be useful for discarding excess input channels by having a `Pass` with
/// less output buffers than its input.
#[derive(Clone, Debug, PartialEq)]
pub struct Pass;

impl Pass {
    const OUT_PORTS: [u32; 1] = [0];
    const IN_PORTS: [u32; 1] = [0];
}

impl Node for Pass {
    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_input_ports(&self) -> &[u32] {
        &Self::IN_PORTS
    }

    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match (port_type, name) {
            (PortType::In, "data") => 0,
            (PortType::Out, "data") => 0,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        let input = match inputs.get(&0) {
            None => return,
            Some(input) => input,
        };

        for (out_buf, in_buf) in output
            .get_mut(&0)
            .expect(NO_PORT)
            .iter_mut()
            .zip(input.buffers())
        {
            out_buf.copy_from_slice(in_buf);
        }
    }
}
