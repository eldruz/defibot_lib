macro_rules! id {
	($($name:ident;)*) => {
		$(
			#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
			pub struct $name(pub u8);
		)*
	}
}

id! {
	PlayerId;
    GameId;
    FtId;
}

#[derive(Debug, PartialEq)]
pub enum FtState {
    // Pending(PlayerId)
    Pending,
    Canceled,
    Confirmed,
}

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub nick: String,
}

#[derive(Debug)]
pub struct Game {
    pub id: GameId,
    pub name: String,
}

#[derive(Debug)]
pub struct Ft {
    pub id: FtId,
    pub game: GameId,
    pub player_a: PlayerId,
    pub player_b: PlayerId,
    pub score_a: u8,
    pub score_b: u8,
    pub state: FtState,
}

impl Ft {
    pub fn new(id: FtId, game: GameId, player_a: PlayerId, player_b: PlayerId, score_a: u8, score_b: u8) -> Ft {
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