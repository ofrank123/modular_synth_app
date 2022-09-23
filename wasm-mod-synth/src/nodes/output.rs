use dasp::graph::{Buffer, Input, Node};

use std::cell::RefCell;
use std::rc::Rc;

use crate::buffer::OutputBuffer;

pub struct OutputNode {
    output_buffer: Rc<RefCell<OutputBuffer>>,
}

impl OutputNode {
    pub fn new(output_buffer: Rc<RefCell<OutputBuffer>>) -> Self {
        OutputNode {
            output_buffer: output_buffer,
        }
    }
}

impl Node for OutputNode {
    fn process(&mut self, inputs: &[Input], _output: &mut [Buffer]) {
        // Write first channel of first input to output buffer
        let mut output_buffer = self.output_buffer.borrow_mut();
        let input = &inputs[0];
        let in_buffers = input.buffers();
        let buffer = &in_buffers[0];
        output_buffer.write(buffer.into_iter());
    }
}
