use crate::{Buffer, Node, port_panic};
use crate::node::{InputPorts, NO_PORT, OutputPorts, ParamValue, PortType};

pub struct MidiNode {
    note_stack: Vec<u32>,
    last_note: u32,
    pulse_low: bool,
}

impl MidiNode {
    const OUT_PORTS: [u32; 2] = [0, 1];
    const IN_PORTS: [u32; 0] = [];

    pub fn new() -> Self {
        MidiNode {
            note_stack: vec![],
            last_note: 64,
            pulse_low: false,
        }
    }

}

impl Node for MidiNode {
    fn get_output_ports(&self) -> &[u32] { &Self::OUT_PORTS }

    fn get_input_ports(&self) -> &[u32] { &Self::IN_PORTS }

    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match (port_type, name) {
            (PortType::Out, "Note") => 0,
            (PortType::Out, "Gate") => 1,
            (t, n) => port_panic!(t, n)
        }
    }

    fn process(&mut self, _inputs: &InputPorts, output: &mut OutputPorts) {
        let note_out = output.get_mut(&0).expect(NO_PORT);

        for note_buffer in note_out {
            if self.note_stack.len() > 0 {
                let note = self.note_stack[self.note_stack.len() - 1] as f32 / 64.0 - 1.0;
                for i in 0..Buffer::LEN {
                    note_buffer[i] = note;
                }
            } else {
                let note = self.last_note as f32 / 64.0 - 1.0;
                for i in 0..Buffer::LEN {
                    note_buffer[i] = note;
                }
            }
        }

        let gate_out = output.get_mut(&1).expect(NO_PORT);

        for gate_buffer in gate_out {
            for i in 0..Buffer::LEN {
                if self.pulse_low {
                    gate_buffer[i] = -1.0;
                    self.pulse_low = false;
                }
                else if self.note_stack.len() > 0 {
                    gate_buffer[i] = 1.0;
                } else {
                    gate_buffer[i] = -1.0;
                }
            }
        }
    }

    fn update_param(&mut self, _name: &str, _param: ParamValue) {
        panic!("Invalid param update on a Midi module")
    }

    fn midi_message(&mut self, note_on: bool, note: u32) {
        if note_on {
            self.note_stack.push(note);
            if !self.note_stack.contains(&note) {
                self.pulse_low = true;
            }
        } else {
            if self.note_stack.len() == 1 {
                self.last_note = self.note_stack[0];
            }
            self.note_stack.remove(self.note_stack.iter().position(|n| n == &note).unwrap());
        }
    }
}
