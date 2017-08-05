extern crate defibot;

use defibot::model::{
    Player,
    Game,
    Ft,
};

use defibot::rules::ResultsPersistence;
use defibot::rules::ResultsRules;

#[derive(Debug)]
pub struct MockData {
    pub results: Vec<Ft>,
    pub players: Vec<Player>,
    pub games: Vec<Game>,
    pub next_id: usize,
}

impl MockData {
    fn new() -> MockData {
        let default_game = Game {name: String::from("BouleFighter")};
        let default_pone = Player {nick: String::from("eldruz")};
        let default_ptwo = Player {nick: String::from("quinonino")};
        MockData { results: vec![], players: vec![default_pone, default_ptwo], games: vec![default_game], next_id: 0}
    }
}

impl ResultsPersistence for MockData {
    fn get_ft(&mut self, ft: usize) -> Option<&mut Ft> {
        self.results.iter_mut().find(|x| x.id == ft)
    }

    fn get_player(&mut self, p_id: &str) -> Option<&mut Player> {
        self.players.iter_mut().find(|x| x.nick == p_id)
    }

    fn get_game(&mut self, g_id: &str) -> Option<&mut Game> {
        self.games.iter_mut().find(|x| x.name == g_id)
    }

    fn add_player(&mut self, player: &str) {
        match self.players.iter().find(|x| x.nick == player) {
            None => self.players.push(Player {nick: player.to_string()}),
            Some(_) => ()
        };
    }

    fn add_game(&mut self, game: &str) {
        match self.games.iter().find(|x| x.name == game) {
            None => self.games.push(Game{name: game.to_string()}),
            Some(_) => ()
        };
    }

    fn register_ft(&mut self, game: &str, player_a: &str, player_b: &str, score_a: u8, score_b: u8) {
        let id = self.next_id;
        let ft = Ft::new(id, Game{name: game.to_string()}, Player{nick: player_a.to_string()}, Player{nick: player_b.to_string()}, score_a, score_b);
        self.add_player(player_a);
        self.add_player(player_b);
        self.results.push(ft);
        self.next_id += 1;
    }
}

impl ResultsRules for MockData {}

fn main() {
    let mut mock_data = MockData::new();

    mock_data.register_ft("BouleFighter", "el poncho", "quinonino", 5, 4);
    {
        let result = mock_data.get_ft(0).expect("AIEAIEAIE");
        match MockData::validate_ft(result, "eldruz", true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
        match MockData::validate_ft(result, "quinonino", true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
    }


    println!("{:?}", mock_data);
}
