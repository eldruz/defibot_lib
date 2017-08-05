use model::*;

pub trait ResultsPersistence {
    fn get_player(&mut self, p_id: &str) -> Option<&mut Player>;
    fn get_game(&mut self, g_id: &str) -> Option<&mut Game>;
    fn get_ft(&mut self, ft: usize) -> Option<&mut Ft>;

    fn get_all_ft(&self) -> Option<&[Ft]>;

    fn get_results_with_game<'a, 'b>(&self, game: &'b str) -> Option<Vec<Ft>>;
    fn get_results_with_player<'a, 'b>(&self, player: &'b str) -> Option<Vec<Ft>>;
    fn get_win_list<'a, 'b>(&self, player: &'b str) -> Option<Vec<Ft>>;

    fn register_ft(&mut self, game: &str, player_a: &str, player_b: &str, score_a: u8, score_b: u8);
    fn add_player(&mut self, player: &str);
    fn add_game(&mut self, game: &str);
}

pub struct ResultsRules {}

impl ResultsRules {
    pub fn validate_ft<'a, 'b>(ft: &'a mut Ft, player: &'b str, confirm: bool) -> Result<&'a Ft, &'static str> {
        if ft.player_b.nick == player && ft.state == FtState::Pending {
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

    pub fn winner(ft: &Ft) -> Result<&Player, &'static str> {
        match ft.state {
            FtState::Pending => Err("The result is not approved yet."),
            FtState::Canceled => Err("The result was canceled."),
            FtState::Confirmed => {
                if ft.score_a > ft.score_b {
                    Ok(&ft.player_a)
                }
                else {
                    Ok(&ft.player_b)
                }
            }
        }
    }

    pub fn is_winner(ft: &Ft, player: &str) -> Option<bool> {
        let player_win = ResultsRules::winner(ft);
        match player_win  {
            Err(_) => None,
            Ok(player_win) => {
                Some(player_win.nick == player)
            }
        }
    }
}
