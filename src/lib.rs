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

// unit: pixels/seconds
static PLAYER_VELOCITY: f32 = 200.0;

// import functions from JS
extern {
    fn clear_stage();
    fn draw_player(x: u16, y: u16);
}

fn build_initial_state(width: u16, height: u16) -> State {
    State::new(width, height)
}

fn move_player(state: &mut State, elapsed_time: f32) {
    let delta_m: u16 = (PLAYER_VELOCITY * elapsed_time) as u16;

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

#[no_mangle]
pub extern fn update_state(elapsed_time: f32) {
    // to be implemented
    let state = &mut STATE.lock().unwrap();

    move_player(state, elapsed_time);
}

#[no_mangle]
pub extern fn init_game() {
    let state: &mut State = &mut STATE.lock().unwrap();
}

#[no_mangle]
pub extern fn toggle_move_up(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_up = enabled != 0
}

#[no_mangle]
pub extern fn toggle_move_down(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_down = enabled != 0
}

#[no_mangle]
pub extern fn toggle_move_left(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_left= enabled != 0
}

#[no_mangle]
pub extern fn toggle_move_right(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.moving_right = enabled != 0
}

#[no_mangle]
pub extern fn toggle_shoot(enabled: u16) {
    let state = &mut STATE.lock().unwrap();

    state.shooting = enabled != 0
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
