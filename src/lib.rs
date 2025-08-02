use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, window, HtmlCanvasElement, CanvasRenderingContext2d};

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once(); // this logs rust panics to the console

    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("main-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.set_fill_style_str("white");
    context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    context.set_stroke_style_str("black");
    context.set_fill_style_str("black");
    context.set_line_width(2.0);

    context.begin_path();
    context.arc(25.0, 25.0, 25.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
    context.fill();

    console_log("Canvas rendered.");
    Ok(())
}

#[wasm_bindgen]
pub fn on_click_section(section: u8) {
    console_log(&format!("Section {} clicked", section));
}

fn console_log(message: &str) {
    console::log_1(&message.into());
}
