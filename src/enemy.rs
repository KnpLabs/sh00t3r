use super::externs::rand;
use state::EnemyState;

// unit: pixels/seconds
static ENEMY_VELOCITY: u16 = 300;

pub fn generate_enemy (stage_width: u16) -> Option<EnemyState> {
    let birth_chance: u16 = (rand() * 100.0) as u16;

    if birth_chance >= 5 {
        return None;
    }

    let x: u16 = (rand() * stage_width as f64) as u16;
    Some(EnemyState::new(x, 0))
}

pub fn move_enemies (enemies: &mut Vec<EnemyState>, elapsed_time: f32, stage_width: u16, stage_height: u16) {
    let delta: u16 = (ENEMY_VELOCITY as f32 * elapsed_time) as u16;

    for enemy in enemies.iter_mut() {
        enemy.y += delta;

        let shift_chance: u16 = (rand() * 100.0) as u16;
        if shift_chance < 25 {
            let direction: bool = shift_chance % 2 == 0;
            if direction && enemy.x + delta < stage_width {
                enemy.x += delta;
            } else if !direction && enemy.x - delta > 0 {
                enemy.x -= delta;
            }
        }
    }

    // free memory for enemies that are off screen
    enemies.retain(|enemy| {
        enemy.y < stage_height
    });
}
