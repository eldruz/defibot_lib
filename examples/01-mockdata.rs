extern crate defibot_lib;

use defibot_lib::model::{Player, Game, DefiRequest};

use defibot_lib::persistence::memory_persistence::MemoryPersistence;
use defibot_lib::rules::request_rules::{RequestRules, RequestPersistence};


fn main() {
    let mut data = MemoryPersistence::new();

    // Creating some players and registering them
    let p_eldruz = Player { nick: String::from("eldruz") };
    let p_joaquin = Player { nick: String::from("joaquin") };
    data.save_player(&p_eldruz);
    data.save_player(&p_joaquin);

    let first_request = DefiRequest::create_defi_request(0,
                                                         0,
                                                         Game::ST,
                                                         &p_eldruz,
                                                         &p_joaquin,
                                                         5,
                                                         3)
            .expect("error creating request");
    let second_request = DefiRequest::create_defi_request(1,
                                                          1,
                                                          Game::GGXRD,
                                                          &p_eldruz,
                                                          &p_joaquin,
                                                          2,
                                                          5)
            .expect("error creating request");
    let third_request = DefiRequest::create_defi_request(2,
                                                         2,
                                                         Game::ST,
                                                         &p_eldruz,
                                                         &p_joaquin,
                                                         4,
                                                         5)
            .expect("error creating request");

    data.save_defi_request(&first_request);
    data.save_defi_request(&second_request);
    data.save_defi_request(&third_request);

    {
        match RequestRules::validate_defi(&mut data, 0, String::from("joaquin"), true) {
            Err(e) => {
                println!("ERROR VALIDATING: {}", e);
                None
            }
            Ok(defi) => {
                println!("Success validating.");
                Some(defi)
            }
        };

        match RequestRules::validate_defi(&mut data, 1, String::from("eldruz"), true) {
            Err(e) => {
                println!("ERROR VALIDATING: {}", e);
                None
            }
            Ok(defi) => {
                println!("Success validating.");
                Some(defi)
            }
        };
    }

    {
        let winner = &data.get_defi_request(0).unwrap();
        let winner = winner.defi.result.winner();
        match winner {
            Err(e) => println!("There's been some kind of mistake: {}", e),
            Ok(player) => println!("Winner is: {}", player.nick),
        }
    }

    {
        let is_winner = &data.get_defi_request(0).unwrap();
        let is_winner = is_winner
            .defi
            .result
            .is_winner(String::from("quinonino"));
        match is_winner {
            None => println!("THINK OF THE CHILDREN"),
            Some(response) => {
                if response {
                    println!("JOAQUIN MURRIETA")
                } else {
                    println!("ESCROC")
                }
            }
        };
    }

    println!("{:?}", data);
}
