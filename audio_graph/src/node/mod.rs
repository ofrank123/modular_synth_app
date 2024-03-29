use crate::{buffer::Buffer, console_log};
use core::fmt;
use std::collections::HashMap;

mod boxed;
mod clockdiv;
mod delay;
mod envelope;
mod filter;
mod graph;
mod lfo;
mod math;
mod midi;
mod oscillator;
mod output;
mod pass;
mod shq;

pub use boxed::{BoxedNode, BoxedNodeSend};
pub use clockdiv::ClockDivNode;
pub use delay::DelayNode;
pub use envelope::EnvelopeNode;
pub use filter::FilterNode;
pub use graph::GraphNode;
pub use lfo::LfoNode;
pub use math::MathNode;
pub use midi::MidiNode;
pub use oscillator::OscNode;
pub use output::OutputSink;
pub use pass::Pass;
pub use shq::ShqNode;

// Map from input port number to Input on that port
pub type InputPorts = HashMap<u32, Input>;
// Map from output port number to buffers being sent out on that port
pub type OutputPorts = HashMap<u32, Vec<Buffer>>;

#[derive(PartialEq, Eq, Hash)]
pub enum PortType {
    In,
    Out,
}

pub const NO_PORT: &'static str = "Unexpected port number encountered";

#[macro_export]
macro_rules! port_panic {
    ($a:expr,$b:expr) => {{
        match ($a, $b) {
            (PortType::In, p) => panic!("No such input port, {}", p),
            (PortType::Out, p) => panic!("No such output port, {}", p),
        }
    }};
}

#[derive(Debug)]
pub enum ParamValue {
    Num(f32),
    Str(String),
}

impl From<f32> for ParamValue {
    fn from(f: f32) -> Self {
        ParamValue::Num(f)
    }
}

impl From<String> for ParamValue {
    fn from(s: String) -> Self {
        ParamValue::Str(s)
    }
}

pub trait Node {
    fn get_output_ports(&self) -> &[u32];
    fn get_input_ports(&self) -> &[u32];
    fn get_port(&self, name: &str, port_type: PortType) -> u32;
    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts);

    fn update_param(&mut self, name: &str, param: ParamValue) {
        console_log!(
            "Node does not handle updates, received: {} {:?}",
            name,
            param
        );
    }

    fn midi_message(&mut self, _note_on: bool, _note: u32) {
        console_log!("Node does not handle midi notes!")
    }
}

pub struct Input {
    buffers_ptr: *const Buffer,
    buffers_len: usize,
}

impl Input {
    pub(crate) fn new(slice: &[Buffer]) -> Self {
        let buffers_ptr = slice.as_ptr();
        let buffers_len = slice.len();
        Input {
            buffers_ptr,
            buffers_len,
        }
    }

    pub fn buffers(&self) -> &[Buffer] {
        unsafe { std::slice::from_raw_parts(self.buffers_ptr, self.buffers_len) }
    }
}

unsafe impl Send for Input {}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.buffers(), f)
    }
}

impl<'a, T> Node for &'a mut T
where
    T: Node + ?Sized,
{
    fn get_output_ports(&self) -> &[u32] {
        (**self).get_output_ports()
    }
    fn get_input_ports(&self) -> &[u32] {
        (**self).get_input_ports()
    }
    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        (**self).get_port(name, port_type)
    }
    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        (**self).process(inputs, output)
    }
}

impl<T> Node for Box<T>
where
    T: Node + ?Sized,
{
    fn get_output_ports(&self) -> &[u32] {
        (**self).get_output_ports()
    }
    fn get_input_ports(&self) -> &[u32] {
        (**self).get_input_ports()
    }
    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        (**self).get_port(name, port_type)
    }
    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        (**self).process(inputs, output)
    }
}
