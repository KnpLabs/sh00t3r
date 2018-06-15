#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

struct State {
    // game: Game,
}

// Lazy static access to the STATE var.
// Use Mutex as JS is single threaded (and rust is not)
lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(build_initial_state(800, 600));
}

// import functions from JS
extern {
    fn clear_stage();
}

fn build_initial_state(width: u16, height: u16) -> State {
    State {
        // game: build_game
    }
}

#[no_mangle]
pub extern fn init_game() {
    let state: &mut State = &mut STATE.lock().unwrap();
}

#[no_mangle]
pub unsafe extern fn render() {
    clear_stage();
}
