use crate::{port_panic, Buffer, Node};

use super::{PortType, NO_PORT};

pub struct ClockDivNode {
    current_div: u32,
    was_low: bool,
}

impl ClockDivNode {
    const OUT_PORTS: [u32; 7] = [0, 1, 2, 3, 4, 5, 6];
    const IN_PORTS: [u32; 1] = [0];

    pub fn new() -> Self {
        ClockDivNode {
            current_div: 1,
            was_low: false,
        }
    }
}

impl Node for ClockDivNode {
    fn get_output_ports(&self) -> &[u32] {
        &Self::OUT_PORTS
    }

    fn get_input_ports(&self) -> &[u32] {
        &Self::IN_PORTS
    }

    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match (port_type, name) {
            (PortType::In, "Clock") => 0,
            (PortType::Out, "1/4") => 0,
            (PortType::Out, "2/4") => 1,
            (PortType::Out, "3/4") => 2,
            (PortType::Out, "4/4") => 3,
            (PortType::Out, "1/3") => 4,
            (PortType::Out, "2/3") => 5,
            (PortType::Out, "3/3") => 6,
            (t, n) => port_panic!(t, n),
        }
    }

    fn process(&mut self, inputs: &super::InputPorts, output: &mut super::OutputPorts) {
        let clock_in = &inputs.get(&0).expect(NO_PORT).buffers()[0];

        for i in 0..Buffer::LEN {
            let mut div_changed = false;

            if self.was_low && (clock_in[i] > 0.0) {
                self.current_div += 1;
                if self.current_div > 12 {
                    self.current_div = 1;
                }

                self.was_low = false;
                div_changed = true;
            }

            if clock_in[i] < 0.0 {
                self.was_low = true;
            }

            let quart_1 = &mut output.get_mut(&0).expect(NO_PORT)[0];
            if self.current_div == 1 && div_changed {
                quart_1[i] = -1.0;
            } else if self.current_div <= 3 {
                quart_1[i] = 1.0
            } else {
                quart_1[i] = -1.0;
            }

            let quart_2 = &mut output.get_mut(&1).expect(NO_PORT)[0];
            if self.current_div == 4 && div_changed {
                quart_2[i] = -1.0;
            } else if self.current_div >= 4 && self.current_div <= 6 {
                quart_2[i] = 1.0;
            } else {
                quart_2[i] = -1.0;
            }

            let quart_3 = &mut output.get_mut(&2).expect(NO_PORT)[0];
            if self.current_div == 7 && div_changed {
                quart_3[i] = -1.0;
            } else if self.current_div >= 7 && self.current_div <= 9 {
                quart_3[i] = 1.0;
            } else {
                quart_3[i] = -1.0;
            }

            let quart_4 = &mut output.get_mut(&3).expect(NO_PORT)[0];
            if self.current_div == 10 && div_changed {
                quart_4[i] = -1.0;
            } else if self.current_div >= 10 {
                quart_4[i] = 1.0;
            } else {
                quart_4[i] = -1.0;
            }

            let tri_1 = &mut output.get_mut(&4).expect(NO_PORT)[0];
            if self.current_div == 1 && div_changed {
                tri_1[i] = -1.0;
            } else if self.current_div <= 4 {
                tri_1[i] = 1.0;
            } else {
                tri_1[i] = -1.0;
            }

            let tri_2 = &mut output.get_mut(&5).expect(NO_PORT)[0];
            if self.current_div == 5 && div_changed {
                tri_2[i] = -1.0;
            } else if self.current_div >= 5 && self.current_div <= 8 {
                tri_2[i] = 1.0;
            } else {
                tri_2[i] = -1.0;
            }

            let tri_3 = &mut output.get_mut(&6).expect(NO_PORT)[0];
            if self.current_div == 9 && div_changed {
                tri_3[i] = -1.0;
            } else if self.current_div >= 9 {
                tri_3[i] = 1.0;
            } else {
                tri_3[i] = -1.0;
            }
        }
    }
}
