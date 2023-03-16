use crate::*;

pub fn create_lfo_spec() -> Module {
    Module {
        name: "lfo",
        rows: vec![
            Row {
                input: Some("Frequency"),
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Frequency",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 1000,
                        order: 4,
                        range: 100.0,
                        inverts: false,
                        default: 10.0,
                        parameter: "base_pitch",
                    }),
                ],
            },
            Row {
                input: Some("Pulse Width"),
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Pulse Width",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 100,
                        order: 1,
                        range: 1.0,
                        inverts: false,
                        default: 0.5,
                        parameter: "pulse_width",
                    }),
                ],
            },
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Type",
                        justify: Justify::Left,
                    }),
                    RowElement::Selector(Selector {
                        options: vec![
                            SelectorOption {
                                name: "Sine",
                                value: "sine",
                            },
                            SelectorOption {
                                name: "Square",
                                value: "square",
                            },
                            SelectorOption {
                                name: "Saw",
                                value: "saw",
                            },
                            SelectorOption {
                                name: "Triangle",
                                value: "tri",
                            },
                            SelectorOption {
                                name: "Noise",
                                value: "noise",
                            },
                        ],
                        parameter: "type",
                    }),
                ],
            },
            Row {
                input: None,
                output: Some("Audio"),
                elements: vec![RowElement::Text(Text {
                    data: "Output",
                    justify: Justify::Left,
                })],
            },
        ],
    }
}
