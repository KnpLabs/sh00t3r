use super::state::{BulletState,EnemyState,State};

pub fn handle_collisions(state: &mut State) {
    handle_player_bullets_collisions(state);
}

fn handle_player_bullets_collisions(state: &mut State) {
    let mut bullet_indexes: Vec<usize> = vec![];
    let mut enemy_indexes: Vec<usize> = vec![];

    for bullet_index in 0..state.bullets.len() {
        if !state.bullets[bullet_index].owned_by_player {
            continue;
        }

        for enemy_index in 0..state.enemies.len() {
            if is_bullet_collision(&state.enemies[enemy_index], &state.bullets[bullet_index]) {
                bullet_indexes.push(bullet_index as usize);
                enemy_indexes.push(enemy_index as usize);
            }
        }
    }

    for bullet_index in bullet_indexes.iter().rev() {
        if bullet_index < &state.bullets.len() {
            state.bullets.remove(*bullet_index);
        }
    }

    for enemy_index in enemy_indexes.iter().rev() {
        if enemy_index < &state.enemies.len() {
            state.enemies.remove(*enemy_index);

            state.score = state.score + 1;
        }
    }
}

fn is_bullet_collision(enemy: & EnemyState, bullet: & BulletState) -> bool {
    return distance(enemy.x, enemy.y, bullet.x, bullet.y) <= (enemy.radius * 2 + bullet.height) as f64;
}

fn distance(x1: u16, y1: u16, x2: u16, y2: u16) -> f64 {
    let dx: u32 = (x2 - x1) as u32;
    let dy: u32 = (y2 - y1) as u32;

    let square_sum: u32 = dx.pow(2) + dy.pow(2);

    return (square_sum as f64).sqrt();
}
