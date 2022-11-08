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
                        min: 0,
                        max: 1000,
                        default: 100,
                        parameter: "base_pitch",
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
