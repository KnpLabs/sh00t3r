#![feature(extern_prelude, type_ascription, proc_macro, wasm_custom_section, wasm_import_module)]

#[macro_use]
extern crate lazy_static;
extern crate wasm_bindgen;

mod state;
mod enemy;
mod lifepack;
pub mod externs;
mod collisions;

use std::cmp;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

use self::state::State;
use self::state::BulletState;
use self::enemy::{generate_enemy, move_enemies};
use self::lifepack::generate_lifepack;

use self::externs::*;
use self::collisions::handle_collisions;

// Lazy static access to the STATE var.
// Use Mutex as JS is single threaded (and rust is not)
lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(build_initial_state(800, 600));
}

// unit: pixels/seconds
static PLAYER_VELOCITY: u16 = 400;
static BULLET_VELOCITY: u16 = 500;
static LIFEPACK_VELOCITY: u16 = 140;

// unit: bullets/seconds
static PLAYER_BULLET_FIRERATE: u16 = 30;
static ENEMY_BULLET_FIRERATE: u16 = 2;

macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn build_initial_state(width: u16, height: u16) -> State {
    State::new(width, height)
}

fn move_player(state: &mut State, elapsed_time: f32) {
    let delta_m: u16 = (PLAYER_VELOCITY as f32 * elapsed_time) as u16;

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

    // constraints player position to world
    state.player.x = cmp::max(10, cmp::min(state.width - 10, state.player.x));
    state.player.y = cmp::max(10, cmp::min(state.height - 10, state.player.y));
}

fn move_bullets(state: &mut State, elapsed_time: f32) {
    let delta_m: u16 = (BULLET_VELOCITY as f32 * elapsed_time) as u16;

    for bullet in state.bullets.iter_mut() {
        if bullet.going_up {
            bullet.y -= delta_m;
        } else {
            bullet.y += delta_m;
        }
    }

    let height = state.height;

    // remove bullets that goes off the screen
    state.bullets.retain(|bullet| {
        bullet.y < height && bullet.y > 0
    });
}

fn move_lifepacks(state: &mut State, elapsed_time: f32) {
    let delta_m: u16 = (LIFEPACK_VELOCITY as f32 * elapsed_time) as u16;

    for lifepack in state.lifepacks.iter_mut() {
        lifepack.y += delta_m;
    }

    let height = state.height;

    // remove lifepacks that goes off the screen
    state.lifepacks.retain(|lifepack| {
        lifepack.y < height
    });
}

fn shoot_bullet(state: &mut State, elapsed_time: f32) {
    let player_shooting_frame = 1.0 / PLAYER_BULLET_FIRERATE as f32;
    let enemy_shooting_frame = 1.0 / ENEMY_BULLET_FIRERATE as f32;

    state.last_shoot_elapsed += elapsed_time;

    if state.shooting && state.last_shoot_elapsed > player_shooting_frame {
        state.bullets.push(BulletState::from_player(&state.player));
        state.last_shoot_elapsed = 0.0;
    }

    for enemy in state.enemies.iter_mut() {
        if enemy.shooting {
            enemy.last_shoot_elapsed += elapsed_time;

            if state.last_shoot_elapsed > enemy_shooting_frame {
                state.bullets.push(BulletState::from_enemy(enemy));
                enemy.last_shoot_elapsed = 0.0;
            }
        }
    }
}

#[wasm_bindgen]
pub extern fn update_state (
    elapsed_time: f32,
    moving_up: u16,
    moving_down: u16,
    moving_left: u16,
    moving_right: u16,
    shooting: u16
) {
    let state = &mut STATE.lock().unwrap();
    let stage_width: u16 = state.width;
    let stage_height: u16 = state.height;

    handle_collisions(state);

    state.moving_up = moving_up != 0;
    state.moving_down = moving_down != 0;
    state.moving_left = moving_left != 0;
    state.moving_right = moving_right != 0;
    state.shooting = shooting != 0;

    move_player(state, elapsed_time);
    move_bullets(state, elapsed_time);
    move_lifepacks(state, elapsed_time);
    move_enemies(&mut state.enemies, elapsed_time, stage_width, stage_height);

    shoot_bullet(state, elapsed_time);

    match generate_enemy(state.width) {
        Some(x) => state.enemies.push(x),
        None => {},
    }

    match generate_lifepack(state) {
        Some(x) => state.lifepacks.push(x),
        None => {}
    }
}

#[wasm_bindgen]
pub extern fn init_game() {
    let state: &mut State = &mut STATE.lock().unwrap();
}

#[wasm_bindgen]
pub extern fn render() -> bool {
    clear_stage();
    println!("Rendering next frame...");

    let state = &mut STATE.lock().unwrap();

    if state.player.life <= 0 {
        draw_game_over(state.score);
        return false;
    }

    draw_player(state.player.x, state.player.y);

    for enemy in state.enemies.iter() {
        draw_enemy(enemy.x, enemy.y, enemy.radius);
    }

    // bullets
    for bullet in state.bullets.iter() {
        draw_bullet(bullet.x, bullet.y);
    }

    // lifepacks
    for lifepack in state.lifepacks.iter() {
        draw_lifepack(lifepack.x, lifepack.y);
    }

    draw_hud(state.player.life, state.score);

    return true;
}
