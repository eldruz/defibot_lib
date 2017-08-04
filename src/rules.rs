use model::*;

pub trait ResultsPersistence {
    fn get_ft(&mut self, ft: FtId) -> Option<&mut Ft>;
    fn register_ft(&mut self, game: GameId, player_a: PlayerId, player_b: PlayerId, score_a: u8, score_b: u8);
}

pub trait ResultsRules {
    fn validate_ft(ft: &mut Ft, player: PlayerId, confirm: bool) -> Result<&Ft, &'static str> {
        if ft.player_b == player && ft.state == FtState::Pending {
            // change the state depending of the confirm
            match confirm {
                false => ft.state = FtState::Canceled,
                true => ft.state = FtState::Confirmed
            }
            Ok(ft)
        }
        else {
            Err("Player was not part of this FT")
        }
    }
}
