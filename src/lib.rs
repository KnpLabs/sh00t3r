#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

mod state;

use std::sync::Mutex;
use self::state::State;
use wasm_bindgen::prelude::*;

// Lazy static access to the STATE var.
// Use Mutex as JS is single threaded (and rust is not)
lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(build_initial_state(800, 600));
}

// unit: pixels/seconds
static PLAYER_VELOCITY: u16 = 200;

#[wasm_bindgen]
extern {
    fn clear_stage();
    fn draw_player(x: u16, y: u16);
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);
}

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn build_initial_state(width: u16, height: u16) -> State {
    State::new(width, height)
}

fn move_player(state: &mut State, elapsed_time: f32) {
    let delta_m: u16 = (PLAYER_VELOCITY as f32 * elapsed_time) as u16;

    // todo : add checks when colliding on the world edges
    if state.moving_right {
        state.player.x = state.player.x + delta_m;
    }

    if state.moving_left {
        state.player.x = state.player.x - delta_m;
    }

    if state.moving_up {
        state.player.y = state.player.y - delta_m;
    }

    if state.moving_down {
        state.player.y = state.player.y + delta_m;
    }
}

#[wasm_bindgen]
pub extern fn update_state(elapsed_time: f32) {
    // to be implemented
    let state = &mut STATE.lock().unwrap();

    move_player(state, elapsed_time);
}

#[wasm_bindgen]
pub extern fn init_game() {
    let state: &mut State = &mut STATE.lock().unwrap();
}

#[wasm_bindgen]
pub extern fn toggle_move_up(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_up = enabled != 0
}

#[wasm_bindgen]
pub extern fn toggle_move_down(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_down = enabled != 0
}

#[wasm_bindgen]
pub extern fn toggle_move_left(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_left = enabled != 0
}

#[wasm_bindgen]
pub extern fn toggle_move_right(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_right = enabled != 0
}

#[wasm_bindgen]
pub extern fn toggle_shoot(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.shooting = enabled != 0
}

#[wasm_bindgen]
pub extern fn render() {
    clear_stage();
    println!("Rendering next frame...");

    let state = &mut STATE.lock().unwrap();

    draw_player(state.player.x, state.player.y);

    // enemies

    // bullets

    // score

    // hud
}
