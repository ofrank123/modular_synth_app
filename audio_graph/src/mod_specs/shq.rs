use crate::*;

pub fn create_shq_spec() -> Module {
    Module {
        name: "Sample and Hold",
        rows: vec![
            Row {
                input: None,
                output: None,
                elements: vec![
                    RowElement::Text(Text {
                        data: "Hold Time (ms)",
                        justify: Justify::Left,
                    }),
                    RowElement::Slider(Slider {
                        order: 3,
                        steps: 1024,
                        range: 1000.0,
                        inverts: false,
                        default: 0.0,
                        parameter: "hold_time",
                    }),
                ],
            },
            Row {
                input: Some("In"),
                output: Some("Out"),
                elements: vec![
                    RowElement::Text(Text {
                        data: "Input",
                        justify: Justify::Left,
                    }),
                    RowElement::Text(Text {
                        data: "Output",
                        justify: Justify::Right,
                    }),
                ],
            },
        ],
    }
}
