use super::externs::rand;
use state::EnemyState;

pub fn generate_enemy (stage_width: u16) -> Option<EnemyState> {
    let birth_chance: u16 = (rand() * 100.0) as u16;

    if birth_chance >= 5 {
        return None;
    }

    let x: u16 = (rand() * stage_width as f64) as u16;

    match (rand() * 100.0) as u16 {
        0...70 => Some(EnemyState::create_light(x, 0)),
        71...95 => Some(EnemyState::create_medium(x, 0)),
        _ => Some(EnemyState::create_heavy(x, 0))
    }
}

pub fn move_enemies (enemies: &mut Vec<EnemyState>, elapsed_time: f32, stage_width: u16, stage_height: u16) {
    for enemy in enemies.iter_mut() {
        let delta = (enemy.velocity as f32 * elapsed_time) as u16;
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
