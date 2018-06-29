use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn clear_stage();
    pub fn draw_player(x: u16, y: u16);
    pub fn draw_bullet(x: u16, y: u16);
    pub fn draw_enemy(x: u16, y: u16, radius: u8);
    pub fn draw_hud(remainingLifes: u8, currentScore: u16);
    pub fn draw_lifepack(x: u16, y: u16);
    pub fn draw_game_over(score: u16);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);
    pub fn rand() -> f64;
}
