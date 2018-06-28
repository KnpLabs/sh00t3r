pub struct State {
    pub width: u16,
    pub height: u16,
    pub score: u16,
    pub player: PlayerState
}

pub struct PlayerState {
    pub x: u16,
    pub y: u16,
    pub life: u8
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
