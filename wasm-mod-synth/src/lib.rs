mod buffer;
mod manager;
mod nodes;
mod utils;

use buffer::OutputBuffer;
use dasp::graph::{Buffer, Input, Node};
use dasp::signal::{self as signal, Signal};
use petgraph::{self as petgraph};
use std::cell::RefCell;
use std::rc::Rc;
use utils::set_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_wasm() {
    set_panic_hook();
}
