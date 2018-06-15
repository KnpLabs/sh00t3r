// import functions from JS
extern {
    fn clear_stage();
}

#[no_mangle]
pub extern fn build_game(width: u16, height: u16) {
    // @TODO : initialize game state
}

#[no_mangle]
pub unsafe extern fn render() {
    clear_stage();
}
