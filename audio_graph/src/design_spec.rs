use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Module {
    pub name: &'static str,
    pub rows: Vec<Row>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Row {
    pub input: Option<ModPort>,
    pub output: Option<ModPort>,
    pub elements: Vec<RowElement>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct ModPort {
    pub name: &'static str,
    pub id: u32,
}

#[derive(Debug, Serialize, Clone, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
pub enum RowElement {
    Text(Text),
    Slider(Slider),
    Selector(Selector),
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct Text {
    pub data: &'static str,
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
