extern crate defibot_lib;

use defibot_lib::model::{
    Player,
    Game,
    Ft,
};

use defibot_lib::rules::ResultsPersistence;
use defibot_lib::rules::ResultsRules;

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
        let another_game = Game {name: String::from("Marvel VS BouleBoule")};
        let default_pone = Player {nick: String::from("eldruz")};
        let default_ptwo = Player {nick: String::from("quinonino")};
        MockData { results: vec![], players: vec![default_pone, default_ptwo], games: vec![default_game, another_game], next_id: 0}
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

    fn get_all_ft(&self) -> Option<&[Ft]> {
        if self.results.is_empty() {
            None
        }
        else {
            Some(&self.results)
        }
    }

    fn get_results_with_game<'a, 'b>(&self, game: &'b str) -> Option<Vec<Ft>> {
        let mut fts: Vec<Ft> = self.results.to_vec();
        fts.retain(|x| x.game.name == game);
        if fts.is_empty() { None } else { Some(fts) }
    }

    fn get_results_with_player<'a, 'b>(&self, player: &'b str) -> Option<Vec<Ft>> {
        let mut fts: Vec<Ft> = self.results.to_vec();
        fts.retain(|x| (x.player_a.nick == player) || (x.player_b.nick == player));
        if fts.is_empty() { None } else { Some(fts) }
    }

    fn get_win_list<'a, 'b>(&self, player: &'b str) -> Option<Vec<Ft>> {
        match self.get_results_with_player(player) {
            None => None,
            Some(games) => {
                let mut wins: Vec<Ft> = games.to_vec();
                wins.retain(|x| x.player_a.nick == player && x.score_a > x.score_b || x.player_b.nick == player && x.score_b > x.score_a);
                if wins.is_empty() { None } else { Some(wins) }
            }
        }
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

fn main() {
    let mut mock_data = MockData::new();

    mock_data.register_ft("BouleFighter", "el poncho", "quinonino", 5, 4);
    mock_data.register_ft("Marvel VS BouleBoule", "el poncho", "quinonino", 5, 0);
    mock_data.register_ft("BouleFighter", "el poncho", "quinonino", 0, 5);
    {
        match ResultsRules::validate_ft(mock_data.get_ft(0).expect("AIEAIEAIE"), "eldruz", true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
        match ResultsRules::validate_ft(mock_data.get_ft(0).expect("AIEAIEAIE"), "quinonino", true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
        match ResultsRules::validate_ft(mock_data.get_ft(1).expect("AIEAIEAIE"), "quinonino", true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
        match ResultsRules::validate_ft(mock_data.get_ft(2).expect("AIEAIEAIE"), "quinonino", true) {
            Err(e) => println!("ERROR : {}", e),
            Ok(_) => println!("Success")
        }
    }

    {
        let winner = ResultsRules::winner(mock_data.get_ft(0).unwrap());
        match winner {
            Err(e) => println!("There's been some kind of mistake: {}", e),
            Ok(player) => println!("Winner is: {}", player.nick)
        }
    }

    {
        match ResultsRules::is_winner(mock_data.get_ft(0).unwrap(), "quinonino") {
            None => println!("THINK OF THE CHILDREN"),
            Some(response) => {
                if response {println!("EL PONCHO")} else {println!("ESCROC")}
            }
        };
    }

    {
        let wins = mock_data.get_win_list("el poncho");
        println!("Wins of el poncho : {:?}", wins.unwrap());
    }

    {
        let game_results = mock_data.get_results_with_game("BouleFighter");
        println!("Defis joués sur BouleFighter: {:?}", game_results.unwrap());
    }


    println!("{:?}", mock_data);
}
