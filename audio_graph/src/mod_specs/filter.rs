use crate::*;

pub fn create_filter_spec() -> Module {
    Module {
        name: "filter",
        rows: vec![
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Falloff",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        steps: 100,
                        order: 1,
                        range: 1.0,
                        inverts: false,
                        default: 0.5,
                        parameter: "falloff",
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
                        justify: Justify::Right,
                    }),
                ],
            },
        ],
    }
}
