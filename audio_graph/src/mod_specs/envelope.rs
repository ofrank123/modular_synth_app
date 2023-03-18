use crate::*;

pub fn create_envelope_spec() -> Module {
    Module {
        name: "envelope",
        rows: vec![
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Attack (ms)",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 1000,
                        order: 3,
                        range: 1000.0,
                        inverts: false,
                        default: 0.0,
                        parameter: "attack",
                    }),
                ],
            },
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Decay (ms)",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 1000,
                        order: 3,
                        range: 1000.0,
                        inverts: false,
                        default: 0.0,
                        parameter: "decay",
                    }),
                ],
            },
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Sustain",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 100,
                        order: 1,
                        range: 1.0,
                        inverts: false,
                        default: 0.5,
                        parameter: "sustain",
                    }),
                ],
            },
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Release (ms)",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 1000,
                        order: 3,
                        range: 1000.0,
                        inverts: false,
                        default: 0.0,
                        parameter: "release",
                    }),
                ],
            },
            Row {
                input: Some("Gate"),
                output: None,
                elements: vec![RowElement::Text(Text {
                    data: "Gate",
                    justify: Justify::Left,
                })],
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
                        justify: Justify::Right,
                    }),
                ],
            },
        ],
    }
}
