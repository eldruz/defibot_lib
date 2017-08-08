extern crate defibot_lib;

use defibot_lib::model::{
    Player,
    Game,
    Defi,
    DefiRequest,
};

use defibot_lib::rules::Persistence;
use defibot_lib::rules::DefiRules;

#[derive(Debug)]
pub struct MockData {
    pub results: Vec<Defi>,
    pub requests: Vec<DefiRequest>,
    pub players: Vec<Player>,
}

impl MockData {
    fn new() -> MockData {
        let default_pone = Player {nick: String::from("eldruz")};
        let default_ptwo = Player {nick: String::from("joaquin")};
        MockData { results: vec![], requests: vec![], players: vec![default_pone, default_ptwo] }
    }
}

#[derive(Debug)]
pub struct MemoryPersistence {
    data: MockData
}

impl MemoryPersistence {
    pub fn new() -> MemoryPersistence {
        MemoryPersistence { data: MockData::new() }
    }
}

impl Persistence for MemoryPersistence {
    fn get_player(&self, p_id: &str) -> Option<Player> {
        match self.data.players.iter().find(|x| x.nick.as_str() == p_id) {
            None => None,
            Some(p) => Some(p.clone())
        }
    }

    // fn get_defi(&self, d_id: usize) -> Option<Defi> {
    //     match self.data.results.iter().find(|x| x.id == d_id) {
    //         None => None,
    //         Some(d) => Some(d.clone())
    //     }
    // }

    // fn get_defi_result(&self, d_id: usize) -> Option<DefiResult> {
    //     match self.get_defi(d_id) {
    //         None => None,
    //         Some(d) => Some(d.result.clone())
    //     }
    // }

    fn get_defi_request(&self, dr_id: usize) -> Option<DefiRequest> {
        match self.data.requests.iter().find(|x| x.id == dr_id) {
            None => None,
            Some(dr) => Some(dr.clone())
        }
    }

    // fn get_all_defi(&self) -> Option<&[Defi]> {
    //     if self.data.results.is_empty() {
    //         None
    //     }
    //     else {
    //         Some(&self.data.results)
    //     }
    // }

    // fn get_all_defi_request(&self) -> Option<&[DefiRequest]> {
    //     if self.data.requests.is_empty() {
    //         None
    //     }
    //     else {
    //         Some(&self.data.requests)
    //     }
    // }

    // fn get_results_with_game<'a, 'b>(&self, game: &'b str) -> Option<Vec<Ft>> {
    //     let mut fts: Vec<Ft> = self.results.to_vec();
    //     fts.retain(|x| x.game.name == game);
    //     if fts.is_empty() { None } else { Some(fts) }
    // }

    // fn get_results_with_player<'a, 'b>(&self, player: &'b str) -> Option<Vec<Ft>> {
    //     let mut fts: Vec<Ft> = self.results.to_vec();
    //     fts.retain(|x| (x.player_a.nick == player) || (x.player_b.nick == player));
    //     if fts.is_empty() { None } else { Some(fts) }
    // }

    // fn get_win_list<'a, 'b>(&self, player: &'b str) -> Option<Vec<Ft>> {
    //     match self.get_results_with_player(player) {
    //         None => None,
    //         Some(games) => {
    //             let mut wins: Vec<Ft> = games.to_vec();
    //             wins.retain(|x| x.player_a.nick == player && x.score_a > x.score_b || x.player_b.nick == player && x.score_b > x.score_a);
    //             if wins.is_empty() { None } else { Some(wins) }
    //         }
    //     }
    // }

    fn save_defi(&mut self, defi: &Defi) {
        match self.data.results.iter().find(|x| x.id == defi.id) {
            None => self.data.results.push(defi.clone()),
            Some(_) => ()
        }
    }

    fn save_defi_request(&mut self, defi_request: &DefiRequest) {
        match self.data.requests.iter().position(|x| x.id == defi_request.id) {
            None => self.data.requests.push(defi_request.clone()),
            Some(pos) => self.data.requests[pos].state = defi_request.state.clone(),
        };
    }

    // fn save_player(&mut self, player: &Player) {
    //     match self.data.players.iter().find(|x| x.nick == player.nick) {
    //         None => self.data.players.push(player.clone()),
    //         Some(_) => ()
    //     };
    // }
}

fn main() {
    let mut memory_persistence = MemoryPersistence::new();

    let p_eldruz = Player { nick: String::from("eldruz") };
    let p_joaquin = Player { nick: String::from("joaquin") };

    let first_request = DefiRequest::create_defi_request(
        0, 0, Game::ST, &p_eldruz, &p_joaquin, 5, 3
    ).expect("error creating request");
    let second_request = DefiRequest::create_defi_request(
        1, 1, Game::GGXRD, &p_eldruz, &p_joaquin, 2, 5
    ).expect("error creating request");
    let third_request = DefiRequest::create_defi_request(
        2, 2, Game::ST, &p_eldruz, &p_joaquin, 4, 5
    ).expect("error creating request");

    memory_persistence.save_defi_request(&first_request);
    memory_persistence.save_defi_request(&second_request);
    memory_persistence.save_defi_request(&third_request);

    {
        match DefiRules::validate_defi(&mut memory_persistence, 0, String::from("joaquin"), true) {
            Err(e) => {
                println!("ERROR VALIDATING: {}", e);
                None
            },
            Ok(defi) => {
                println!("Success validating.");
                Some(defi)
            }
        };

        match DefiRules::validate_defi(&mut memory_persistence, 1, String::from("eldruz"), true) {
            Err(e) => {
                println!("ERROR VALIDATING: {}", e);
                None
            },
            Ok(defi) => {
                println!("Success validating.");
                Some(defi)
            }
        };
    }
    
    // {
    //     let winner = &memory_persistence.get_defi_result(0).unwrap();
    //     let winner = DefiRules::winner(winner);
    //     match winner {
    //         Err(e) => println!("There's been some kind of mistake: {}", e),
    //         Ok(player) => println!("Winner is: {}", player.nick)
    //     }
    // }

    // {
    //     match ResultsRules::is_winner(mock_data.get_ft(0).unwrap(), "quinonino") {
    //         None => println!("THINK OF THE CHILDREN"),
    //         Some(response) => {
    //             if response {println!("EL PONCHO")} else {println!("ESCROC")}
    //         }
    //     };
    // }

    // {
    //     let wins = mock_data.get_win_list("el poncho");
    //     println!("Wins of el poncho : {:?}", wins.unwrap());
    // }

    // {
    //     let game_results = mock_data.get_results_with_game("BouleFighter");
    //     println!("Defis joués sur BouleFighter: {:?}", game_results.unwrap());
    // }


    println!("{:?}", memory_persistence);
}
