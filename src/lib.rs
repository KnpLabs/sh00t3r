#[macro_use]
extern crate lazy_static;

mod state;

use std::sync::Mutex;
use self::state::State;

// Lazy static access to the STATE var.
// Use Mutex as JS is single threaded (and rust is not)
lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(build_initial_state(800, 600));
}

// import functions from JS
extern {
    fn clear_stage();
    fn draw_player(x: u16, y: u16);
}

fn build_initial_state(width: u16, height: u16) -> State {
    State::new(width, height)
}

#[no_mangle]
pub extern fn update_state(elapsed_time: f32) {
    // to be implemented
}

#[no_mangle]
pub extern fn init_game() {
    let state: &mut State = &mut STATE.lock().unwrap();
}

#[no_mangle]
pub extern fn toggle_move_up() {
    // to be implemented
}

#[no_mangle]
pub extern fn toggle_move_down() {
    // to be implemented
}

#[no_mangle]
pub extern fn toggle_move_left() {
    // to be implemented
}

#[no_mangle]
pub extern fn toggle_move_right() {
    // to be implemented
}

#[no_mangle]
pub extern fn toggle_shoot() {
    // to be implemented
}

#[no_mangle]
pub unsafe extern fn render() {
    clear_stage();

    let state = &mut STATE.lock().unwrap();

    draw_player(state.player.x, state.player.y);

    // enemies

    // bullets

    // score

    // hud
}
