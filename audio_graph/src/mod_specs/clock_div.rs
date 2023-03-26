use crate::*;

pub fn create_clock_div_spec() -> Module {
    Module {
        name: "clock divider",
        rows: vec![
            Row {
                input: Some("Clock"),
                output: Some("1/4"),
                elements: vec![
                    RowElement::Text(Text {
                        data: "Clock",
                        justify: Justify::Left,
                    }),
                    RowElement::Text(Text {
                        data: "1/4",
                        justify: Justify::Right,
                    }),
                ],
            },
            Row {
                input: None,
                output: Some("2/4"),
                elements: vec![RowElement::Text(Text {
                    data: "2/4",
                    justify: Justify::Right,
                })],
            },
            Row {
                input: None,
                output: Some("3/4"),
                elements: vec![RowElement::Text(Text {
                    data: "3/4",
                    justify: Justify::Right,
                })],
            },
            Row {
                input: None,
                output: Some("4/4"),
                elements: vec![RowElement::Text(Text {
                    data: "4/4",
                    justify: Justify::Right,
                })],
            },
            Row {
                input: None,
                output: Some("1/3"),
                elements: vec![RowElement::Text(Text {
                    data: "1/3",
                    justify: Justify::Right,
                })],
            },
            Row {
                input: None,
                output: Some("2/3"),
                elements: vec![RowElement::Text(Text {
                    data: "2/3",
                    justify: Justify::Right,
                })],
            },
            Row {
                input: None,
                output: Some("3/3"),
                elements: vec![RowElement::Text(Text {
                    data: "3/3",
                    justify: Justify::Right,
                })],
            },
        ],
    }
}
