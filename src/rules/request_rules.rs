use model::*;

pub trait RequestPersistence {
    fn get_player(&self, p_id: &str) -> Option<Player>;
    fn get_defi_request(&self, dr_id: usize) -> Option<DefiRequest>;

    fn save_defi(&mut self, defi: &Defi);
    fn save_defi_request(&mut self, defi_request: &DefiRequest);
    fn save_player(&mut self, player: &Player);
}

pub struct RequestRules {}

impl RequestRules {
    pub fn validate_defi<T>(gateway: &mut T, dr_id: usize, player_name: String, confirm: bool) -> Result<Defi, &'static str>
        where T: RequestPersistence
    {
        let request_opt = gateway.get_defi_request(dr_id);
        let player_opt = gateway.get_player(player_name.as_str());

        match (request_opt, player_opt) {
            (_, None) => {
                Err("Player not found.")
            },
            (None, _) => {
                Err("Request not found.")
            }
            (Some(mut request), Some(player)) => {
                if request.defi.result.player_b.nick == player.nick && request.state == DefiState::Pending {
                    match confirm {
                        false => request.change_state(DefiState::Canceled),
                        true => request.change_state(DefiState::Confirmed)
                    }
                    gateway.save_defi_request(&request);
                    gateway.save_defi(&request.defi);
                    Ok(request.defi.clone())
                }
                else if request.defi.result.player_a.nick == player.nick {
                    Err("Player cannot auto-validate")
                }
                else {
                    Err("Player was not part of this FT")
                }
            }
        }
    }

    pub fn register_request<T>(gateway: &mut T, id_request: usize,
    id_defi: usize, game: Game, player_a: &Player, player_b: &Player, score_a: usize, score_b: usize)
    -> Result<DefiRequest, &'static str>
        where T: RequestPersistence
    {
        let request = DefiRequest::create_defi_request (
            id_request, id_defi, game, player_a, player_b, score_a, score_b
        );
        match request {
            Err(e) => Err(e),
            Ok(correct_request) => {
                gateway.save_player(player_a);
                gateway.save_player(player_b);
                gateway.save_defi_request(&correct_request.clone());
                Ok(correct_request)
            }
        }
    }
}
