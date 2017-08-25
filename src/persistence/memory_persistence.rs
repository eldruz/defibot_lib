use model::{Player, Defi, DefiRequest};

use rules::request_rules::RequestPersistence;

#[derive(Debug)]
pub struct MockData {
    pub results: Vec<Defi>,
    pub requests: Vec<DefiRequest>,
    pub players: Vec<Player>,
}

impl MockData {
    fn new() -> MockData {
        MockData {
            results: vec![],
            requests: vec![],
            players: vec![],
        }
    }
}

#[derive(Debug)]
pub struct MemoryPersistence {
    data: MockData,
}

impl MemoryPersistence {
    pub fn new() -> MemoryPersistence {
        MemoryPersistence { data: MockData::new() }
    }
}

impl RequestPersistence for MemoryPersistence {
    fn get_player(&self, p_id: &str) -> Option<Player> {
        match self.data
                  .players
                  .iter()
                  .find(|x| x.nick.as_str() == p_id) {
            None => None,
            Some(p) => Some(p.clone()),
        }
    }

    fn get_defi_request(&self, dr_id: usize) -> Option<DefiRequest> {
        match self.data.requests.iter().find(|x| x.id == dr_id) {
            None => None,
            Some(dr) => Some(dr.clone()),
        }
    }

    fn save_defi(&mut self, defi: &Defi) {
        match self.data.results.iter().find(|x| x.id == defi.id) {
            None => self.data.results.push(defi.clone()),
            Some(_) => (),
        }
    }

    fn save_defi_request(&mut self, defi_request: &DefiRequest) {
        match self.data
                  .requests
                  .iter()
                  .position(|x| x.id == defi_request.id) {
            None => self.data.requests.push(defi_request.clone()),
            Some(pos) => self.data.requests[pos].state = defi_request.state.clone(),
        };
    }

    fn save_player(&mut self, player: &Player) {
        match self.data
                  .players
                  .iter()
                  .find(|x| x.nick == player.nick) {
            None => self.data.players.push(player.clone()),
            Some(_) => (),
        };
    }
}
