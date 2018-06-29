pub struct State {
    pub width: u16,
    pub height: u16,
    pub score: u16,
    pub player: PlayerState,
    pub bullets: Vec<BulletState>,
    pub enemies: Vec<EnemyState>,
    pub lifepacks: Vec<LifepackState>,
    pub moving_up: bool,
    pub moving_down: bool,
    pub moving_right: bool,
    pub moving_left: bool,
    pub shooting: bool,
    pub last_shoot_elapsed: f32
}

pub struct PlayerState {
    pub x: u16,
    pub y: u16,
    pub life: u8
}

pub struct BulletState {
    pub x: u16,
    pub y: u16,
    pub going_up: bool,
    pub owned_by_player: bool
}

pub struct EnemyState {
    pub x: u16,
    pub y: u16
}

pub struct LifepackState {
    pub x: u16,
    pub y: u16,
    pub life_unit: u8
}

impl State {
    pub fn new (width: u16, height: u16) -> State {
        State {
            width,
            height,
            score: 0,
            player: PlayerState::new(width, height),
            bullets: Vec::new(),
            enemies: vec![],
            lifepacks: Vec::new(),
            moving_up: false,
            moving_down: false,
            moving_right: false,
            moving_left: false,
            shooting: false,
            last_shoot_elapsed: 0.0
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

impl BulletState {
    pub fn new (x: u16, y: u16, going_up: bool, owned_by_player: bool) -> BulletState {
        BulletState {
            x,
            y,
            going_up,
            owned_by_player
        }
    }

    pub fn from_player (player_state: &PlayerState) -> BulletState {
        BulletState::new(player_state.x, player_state.y - 13, true, true)
    }
}

impl EnemyState {
    pub fn new (x: u16, y: u16) -> EnemyState {
        EnemyState {
            x,
            y
        }
    }
}

impl LifepackState {
    pub fn new (x: u16, y: u16) -> LifepackState {
        LifepackState {x, y, life_unit: 1}
    }
}
