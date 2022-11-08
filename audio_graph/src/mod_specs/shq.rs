use crate::*;

pub fn create_shq_spec() -> Module {
    Module {
        name: "shq",
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
                        min: 0,
                        max: 1000,
                        default: 0,
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
