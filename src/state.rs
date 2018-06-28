pub struct State {
    width: u16,
    height: u16,
    score: u16,
    player: PlayerState
}

struct PlayerState {
    x: u16,
    y: u16,
    life: u8
}

impl State {
    pub fn new (width: u16, height: u16) -> State {
        State {
            width,
            height,
            score: 0,
            player: PlayerState::new(width, height)
        }
    }
}

impl PlayerState {
    pub fn new (width: u16, height: u16) -> PlayerState {
        PlayerState {
            x: width / 2,
            y: height - 100,
            life: 5
        }
    }
}
