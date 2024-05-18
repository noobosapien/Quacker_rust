#[macro_use]
mod engine;
mod game;
mod math;
mod segments;
mod sound;

mod geometry;

use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    Ok(())
}
