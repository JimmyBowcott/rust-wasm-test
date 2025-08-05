mod canvas;
mod game;
mod utils;

use crate::game::Game;
use gloo_render::request_animation_frame;
use utils::console_log;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log("starting...");
    utils::set_panic_hook();

    let canvas = canvas::Canvas::new("main-canvas")?;
    let game = Rc::new(RefCell::new(Game::new(canvas)));

    schedule_frame(game);

    console_log("ok");
    Ok(())
}

fn schedule_frame(game: Rc<RefCell<Game>>) {
    request_animation_frame(move |_| {
        game.borrow_mut().update();
        game.borrow().render();
        schedule_frame(game.clone());
    });
}

