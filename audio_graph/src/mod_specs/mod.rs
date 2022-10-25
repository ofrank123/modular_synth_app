use serde::Serialize;
use ts_rs::TS;

mod math;
mod oscillator;

use math::create_math_spec;
use oscillator::create_oscillator_spec;

pub fn get_serialized_specs() -> String {
    let osc_spec = create_oscillator_spec();
    let math_spec = create_math_spec();

    serde_json::to_string(&AllModules {
        data: vec![osc_spec, math_spec],
    })
    .unwrap()
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct AllModules {
    pub data: Vec<Module>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Module {
    pub name: &'static str,
    pub rows: Vec<Row>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Row {
    pub input: Option<&'static str>,
    pub output: Option<&'static str>,
    pub elements: Vec<RowElement>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
#[ts(export)]
pub enum RowElement {
    Text(Text),
    Slider(Slider),
    Selector(Selector),
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
#[ts(export)]
pub enum Justify {
    Left,
    Right,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Text {
    pub data: &'static str,
    pub justify: Justify,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Slider {
    pub max: i32,
    pub min: i32,
    pub default: i32,
    pub parameter: &'static str,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Selector {
    pub options: Vec<SelectorOption>,
    pub parameter: &'static str,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct SelectorOption {
    pub name: &'static str,
    pub value: &'static str,
}
