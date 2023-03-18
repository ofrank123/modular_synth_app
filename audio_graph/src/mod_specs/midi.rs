use crate::*;

pub fn create_midi_spec() -> Module {
    Module {
        name: "midi",
        rows: vec![
            Row {
                input: None,
                output: Some("Note"),
                elements: vec![
                    RowElement::Text(Text {
                        data: "Note",
                        justify: Justify::Right,
                    })
                ]
            },
            Row {
                input: None,
                output: Some("Gate"),
                elements: vec![
                    RowElement::Text(Text {
                        data: "Gate",
                        justify: Justify::Right,
                    })
                ]
            }
        ]
    }
}