extern crate defibot;

use defibot::model::{
    PlayerId,
    GameId,
    FtId,
    FtState,
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
    pub next_id: u8,
}

impl MockData {
    fn new() -> MockData {
        let default_game = Game {id: GameId(0), name: String::from("BouleFighter")};
        let default_pone = Player {id: PlayerId(0), nick: String::from("eldruz")};
        let default_ptwo = Player {id: PlayerId(1), nick: String::from("quinonino")};
        MockData { results: vec![], players: vec![default_pone, default_ptwo], games: vec![default_game], next_id: 0}
    }
}

impl ResultsPersistence for MockData {
    fn get_ft(&mut self, ft: FtId) -> Option<&mut Ft> {
        self.results.iter_mut().find(|x| x.id == ft)
    }

    fn register_ft(&mut self, game: GameId, player_a: PlayerId, player_b: PlayerId, score_a: u8, score_b: u8) {
        let id = FtId(self.next_id);
        let ft = Ft::new(id, game, player_a, player_b, score_a, score_b);
        self.results.push(ft);
        self.next_id += 1;
    }
}

impl ResultsRules for MockData {}

fn main() {
    let mut mock_data = MockData::new();

    mock_data.register_ft(GameId(0), PlayerId(1), PlayerId(0), 5, 4);
    {
        let result = mock_data.get_ft(FtId(0)).expect("AIEAIEAIE");
        match MockData::validate_ft(result, PlayerId(0), true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
    }


    println!("{:?}", mock_data);
}
