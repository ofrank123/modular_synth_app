use serde::Serialize;
use ts_rs::TS;

mod clock_div;
mod delay;
mod envelope;
mod filter;
mod lfo;
mod math;
mod midi;
mod oscillator;
mod shq;

use clock_div::create_clock_div_spec;
use delay::create_delay_spec;
use envelope::create_envelope_spec;
use filter::create_filter_spec;
use lfo::create_lfo_spec;
use math::create_math_spec;
use midi::create_midi_spec;
use oscillator::create_oscillator_spec;
use shq::create_shq_spec;

use crate::{
    node::{
        ClockDivNode, DelayNode, EnvelopeNode, FilterNode, LfoNode, MathNode, MidiNode, OscNode,
        ShqNode,
    },
    BoxedNode, Node,
};

pub struct ModParams {
    pub sample_rate: f64,
}

// Modules must be registered here
pub fn get_serialized_specs() -> String {
    let osc_spec = create_oscillator_spec();
    let math_spec = create_math_spec();
    let lfo_spec = create_lfo_spec();
    let shq_spec = create_shq_spec();
    let delay_spec = create_delay_spec();
    let filter_spec = create_filter_spec();
    let envelope_spec = create_envelope_spec();
    let midi_spec = create_midi_spec();
    let clock_div_spec = create_clock_div_spec();

    serde_json::to_string(&AllModules {
        data: vec![
            osc_spec,
            math_spec,
            lfo_spec,
            shq_spec,
            delay_spec,
            filter_spec,
            envelope_spec,
            midi_spec,
            clock_div_spec,
        ],
    })
    .unwrap()
}

// And here
pub fn new_mod(t_name: &str, mod_params: ModParams) -> BoxedNode {
    match t_name {
        "math" => BoxedNode::new(MathNode::new()),
        "oscillator" => BoxedNode::new(OscNode::new(mod_params.sample_rate)),
        "lfo" => BoxedNode::new(LfoNode::new(mod_params.sample_rate)),
        "Sample and Hold" => BoxedNode::new(ShqNode::new(mod_params.sample_rate)),
        "delay" => BoxedNode::new(DelayNode::new(mod_params.sample_rate)),
        "filter" => BoxedNode::new(FilterNode::new()),
        "envelope" => BoxedNode::new(EnvelopeNode::new(mod_params.sample_rate)),
        "midi" => BoxedNode::new(MidiNode::new()),
        "clock divider" => BoxedNode::new(ClockDivNode::new()),
        _ => panic!("No such module"),
    }
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct AllModules {
    pub data: Vec<Module>,
}

trait SizedNode: Node + Sized {}

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
    pub steps: i32,
    pub order: i32,
    pub range: f32,
    pub inverts: bool,
    pub default: f32,
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
