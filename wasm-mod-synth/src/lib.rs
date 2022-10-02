mod manager;
mod messages;
mod utils;

use utils::set_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_wasm() {
    set_panic_hook();
}
