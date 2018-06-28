use super::externs::rand;
use state::EnemyState;

pub fn generate_enemy (stageWidth: u16) -> Option<EnemyState> {
    let birthChance: u16 = (rand() * 100.0) as u16;

    if birthChance >= 5 {
        return None;
    }

    let x: u16 = (rand() * stageWidth as f64) as u16;
    Some(EnemyState::new(x, 0))
}
