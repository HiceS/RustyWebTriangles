use std::string;

use wasm_bindgen::prelude::*;
use web_sys::console;
use world::World;

mod world;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

// Called by our JS entry point to run the example.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let sim_world = world::setup();

    let size = sim_world.world_data.get_body_count();

    console::log_1(&JsValue::from_str(&size.to_string()));

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn create_world() -> World {
    
    let sim_world = world::setup();

    let size = sim_world.world_data.get_body_count();

    console::log_1(&JsValue::from_str(&size.to_string()));

    return sim_world;
}