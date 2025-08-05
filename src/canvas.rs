use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window};

pub struct Canvas {
    pub ctx: CanvasRenderingContext2d,
    pub width: f64,
    pub height: f64,
}

impl Canvas {
    pub fn new(id: &str) -> Result<Self, wasm_bindgen::JsValue> {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .get_element_by_id(id)
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()?;

        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;

        Ok(Self { ctx, width, height })
    }
}

