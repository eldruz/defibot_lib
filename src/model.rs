#[derive(Debug, PartialEq, Clone)]
pub enum DefiState {
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
pub enum Game {
    ST,
    SF5,
    GGXRD,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefiResult {
    pub player_a: Player,
    pub player_b: Player,
    pub score_a: usize,
    pub score_b: usize,
}

#[derive(Debug, Clone)]
pub struct Defi {
    pub id: usize,
    pub game: Game,
    pub result: DefiResult,
}

#[derive(Debug, Clone)]
pub struct DefiRequest {
    pub id: usize,
    pub defi: Defi,
    pub player_name: String,
    pub state: DefiState,
}

impl DefiRequest {
    fn check_defi (&self) -> Result<&DefiRequest, &'static str> {
        if self.defi.result.score_a == self.defi.result.score_b {
            Err("Cannot have the same score")
        }
        else if self.state != DefiState::Pending {
            Err("Defi cannot be created as non pending")
        }
        else if self.defi.result.player_a == self.defi.result.player_b {
            Err("Defi is between two different people")
        }
        else {
            Ok(self)
        }
    }

    pub fn create_defi_request (id_request: usize,
    id_defi: usize, game: Game, player_a: &Player, player_b: &Player, score_a: usize, score_b: usize)
    -> Result<DefiRequest, &'static str> {
        let defi_request = DefiRequest {
            id: id_request,
            defi: Defi {
                id: id_defi,
                game,
                result: DefiResult {
                    player_a: player_a.clone(),
                    player_b: player_b.clone(),
                    score_a,
                    score_b
                }
            },
            player_name: player_b.nick.clone(),
            state: DefiState::Pending,
        };

        match defi_request.check_defi() {
            Err(e) => Err(e),
            Ok(_) => Ok(defi_request)
        }
    }

    pub fn change_state(&mut self, new_state: DefiState) {
        self.state = new_state;
    }
}