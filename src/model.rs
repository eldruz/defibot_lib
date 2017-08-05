#[derive(Debug, PartialEq, Clone)]
pub enum FtState {
    // Pending(PlayerId)
    Pending,
    Canceled,
    Confirmed,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    pub nick: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Game {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Ft {
    pub id: usize,
    pub game: Game,
    pub player_a: Player,
    pub player_b: Player,
    pub score_a: u8,
    pub score_b: u8,
    pub state: FtState,
}

impl Ft {
    pub fn new(id: usize, game: Game, player_a: Player, player_b: Player, score_a: u8, score_b: u8) -> Ft {
        // for now we assume that player_a is the initiator
        Ft {
            id,
            game,
            player_a,
            player_b,
            score_a,
            score_b,
            state: FtState::Pending,
            }
    }
}