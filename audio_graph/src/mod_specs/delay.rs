use crate::*;

pub fn create_delay_spec() -> Module {
    Module {
        name: "delay",
        rows: vec![
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Delay Length (s)",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 1024,
                        order: 1,
                        range: 1024.0,
                        inverts: false,
                        default: 512.0,
                        parameter: "length_s",
                    }),
                ],
            },
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Delay Length (ms)",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 10000,
                        order: 3,
                        range: 5000.0,
                        inverts: false,
                        default: 1000.0,
                        parameter: "length_ms",
                    }),
                ],
            },
            Row {
                input: Some("Audio"),
                output: Some("Audio"),
                elements: vec![
                    RowElement::Text(Text {
                        data: "Audio",
                        justify: Justify::Left,
                    }),
                    RowElement::Text(Text {
                        data: "Audio",
                        justify: Justify::Right
                    })
                ],
            },
        ],
    }
}
