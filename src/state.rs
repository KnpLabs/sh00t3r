pub struct State {
    pub width: u16,
    pub height: u16,
    pub score: u16,
    pub player: PlayerState,
    pub moving_up: bool,
    pub moving_down: bool,
    pub moving_right: bool,
    pub moving_left: bool,
    pub shooting: bool
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
            player: PlayerState::new(width, height),
            moving_up: false,
            moving_down: false,
            moving_right: false,
            moving_left: false,
            shooting: false
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
