use super::externs::rand;
use state::State;
use state::LifepackState;

pub fn generate_lifepack (state: &mut State) -> Option<LifepackState> {
    if state.player.life >= 2 {
        return None;
    }

    if state.enemies.len() <= 5 {
        return None;
    }

    if state.lifepacks.len() >= 1 {
        return None
    }

    let birth_chance: u16 = (rand() * 100.0) as u16;

    if birth_chance >= 1 {
        return None;
    }

    let x: u16 = (rand() * state.width as f64) as u16;
    Some(LifepackState::new(x, 0))
}
