use model::{Defi, DefiState, DefiResult, DefiRequest, Game, Player};

pub trait CommandPersistence {
    fn get_player(&self, p_id: &str) -> Option<Player>;
    fn get_or_create_player(&self, p_id: &str) -> Player;
    fn get_game(&self, g_id: &str) -> Option<Game>;
    fn get_defi_request(&self, dr_id: usize) -> Result<DefiRequest, &'static str>;
    fn get_pending_defis(&self) -> Result<Vec<Defi>, &'static str>;
    fn get_pending_defis_for_player(&self, p_id: &str) -> Result<Vec<Defi>, &'static str>;
    fn get_pending_defis_for_game(&self, g_id: &str) -> Result<Vec<Defi>, &'static str>;
    fn get_pending_defis_for_player_and_game(&self,
                                             player_name: &str,
                                             game_name: &str)
                                             -> Result<Vec<Defi>, &'static str>;
    fn get_next_defi_request_id(&self) -> usize;
    fn get_next_defi_id(&self) -> usize;

    fn save_defi_request(&self, d_r: DefiRequest) -> Result<DefiRequest, &'static str>;
    fn update_defi_request(&self, d_r: DefiRequest) -> Result<(), &'static str>;

    fn exists_player(&self, p_id: &str) -> bool;
    fn exists_game(&self, g_id: &str) -> bool;
}

#[derive(Debug, Clone)]
pub struct DefiCommand {
    pub player_a: String,
    pub player_b: String,
    pub game: String,
    pub score_a: usize,
    pub score_b: usize,
}

impl DefiCommand {
    pub fn run<T>(&self, gateway: &mut T) -> Result<DefiRequest, &'static str>
        where T: CommandPersistence
    {
        // Get the next ids for insertion in the persistence system
        let id_request = gateway.get_next_defi_request_id();
        let id_defi = gateway.get_next_defi_id();
        // If the players do not exist, create them
        let player_a = gateway.get_or_create_player(self.player_a.as_str());
        let player_b = gateway.get_or_create_player(self.player_b.as_str());

        // Create the request
        let defi_req = DefiRequest::create_defi_request(
            id_request,
            id_defi,
            Game::ST,
            &player_a,
            &player_b,
            self.score_a,
            self.score_b)?;

        // Store it in the persistence system
        gateway.save_defi_request(defi_req)
    }
}

#[derive(Debug, Clone)]
pub struct PendingCommand {
    player_name: Option<String>,
    game: Option<String>,
}

impl PendingCommand {
    pub fn run<T>(&self, gateway: &mut T) -> Result<Vec<Defi>, &'static str>
        where T: CommandPersistence
    {
        match (self.player_name.clone(), self.game.clone()) {
            (None, None) => gateway.get_pending_defis(),
            (Some(player), None) => gateway.get_pending_defis_for_player(player.as_str()),
            (None, Some(game)) => gateway.get_pending_defis_for_game(game.as_str()),
            (Some(p), Some(g)) => {
                gateway.get_pending_defis_for_player_and_game(p.as_str(), g.as_str())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConfirmCommand {
    player_name: String,
    defi_id: usize,
}

impl ConfirmCommand {
    pub fn run<T>(&self, gateway: &mut T) -> Result<(), &'static str>
        where T: CommandPersistence
    {
        let mut defi_request = gateway.get_defi_request(self.defi_id)?;
        let _ret = match defi_request.state {
            DefiState::Pending => {
                if defi_request.player_name == self.player_name {
                    defi_request.change_state(DefiState::Confirmed);
                    gateway.update_defi_request(defi_request)
                } else {
                    Err("The second player does not match.")
                }
            }
            DefiState::Confirmed => Err("The result has already been confirmed."),
            DefiState::Canceled => Err("The result has been canceled."),
        };
        _ret
    }
}
