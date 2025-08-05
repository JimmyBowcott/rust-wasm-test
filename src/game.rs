use crate::{canvas::Canvas, utils::console_log};

pub struct Game {
    canvas: Canvas,
    // game state fields go here
}

impl Game {
    pub fn new(canvas: Canvas) -> Self {
        Self { canvas }
    }

    pub fn update(&mut self) {
        // update game state here
        console_log("game loop");
    }

    pub fn render(&self) {
        console_log("is this getting called?");
        let ctx = &self.canvas.ctx;

        ctx.set_fill_style_str("green");
        ctx.fill_rect(0.0, 0.0, self.canvas.width, self.canvas.height);

        ctx.set_stroke_style_str("black");
        ctx.set_fill_style_str("black");
        ctx.set_line_width(2.0);
        ctx.set_fill_style_str("black");
        ctx.begin_path();
        ctx.arc(50.0, 50.0, 25.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.fill();
    }
}

