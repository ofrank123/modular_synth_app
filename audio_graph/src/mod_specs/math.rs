use crate::{Justify, Module, Row, RowElement, Slider, Text};

macro_rules! get_av_row {
    ( $idx:expr ) => {{
        Row {
            input: Some(concat!("In ", $idx)),
            output: Some(concat!("Out ", $idx)),
            elements: vec![
                RowElement::Text(Text {
                    data: stringify!($idx),
                    justify: Justify::Left,
                }),
                RowElement::Slider(Slider {
                    steps: 100,
                    order: 4,
                    range: 1.0,
                    inverts: true,
                    default: 0.0,
                    parameter: concat!("attenuverter", $idx),
                }),
            ],
        }
    }};
}

pub fn create_math_spec() -> Module {
    Module {
        name: "math",
        rows: vec![
            get_av_row!(1),
            get_av_row!(2),
            get_av_row!(3),
            get_av_row!(4),
            Row {
                input: None,
                output: Some("Sum"),
                elements: vec![RowElement::Text(Text {
                    data: "Sum",
                    justify: Justify::Right,
                })],
            },
        ],
    }
}
