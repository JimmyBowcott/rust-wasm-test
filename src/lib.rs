use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn start() {
    console_log("Loaded from WASM!");
}

#[wasm_bindgen]
pub fn on_click_section(section: u8) {
    console_log(&format!("Section {} clicked", section));
}

fn console_log(message: &str) {
    console::log_1(&message.into());
}
