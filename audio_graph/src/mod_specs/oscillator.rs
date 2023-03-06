use crate::{Module, Row, RowElement, Selector, SelectorOption, Slider, Text};

use super::Justify;

pub fn create_oscillator_spec() -> Module {
    Module {
        name: "oscillator",
        rows: vec![
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Pitch",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        order: 1,
                        steps: 512,
                        range: 1.0,
                        inverts: true,
                        default: 0.0,
                        parameter: "base_pitch",
                    }),
                ],
            },
            Row {
                input: Some("Coarse Pitch"),
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Coarse Pitch",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        order: 1,
                        steps: 512,
                        range: 12.0,
                        inverts: true,
                        default: 0.0,
                        parameter: "coarse_pitch",
                    }),
                ],
            },
            Row {
                input: Some("Fine Pitch"),
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Fine Pitch",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        order: 1,
                        steps: 200,
                        range: 100.0,
                        inverts: true,
                        default: 0.0,
                        parameter: "fine_pitch",
                    }),
                ],
            },
            Row {
                input: Some("Phase"),
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Phase",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        order: 1,
                        steps: 200,
                        range: 1.0,
                        inverts: true,
                        default: 0.0,
                        parameter: "phase",
                    }),
                ],
            },
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Wave",
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
                                name: "Naive Square",
                                value: "nsquare",
                            },
                            SelectorOption {
                                name: "Saw",
                                value: "saw",
                            },
                            SelectorOption {
                                name: "Naive Saw",
                                value: "nsaw",
                            },
                            SelectorOption {
                                name: "Triangle",
                                value: "tri",
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
                    data: "Audio",
                    justify: Justify::Right,
                })],
            },
        ],
    }
}
