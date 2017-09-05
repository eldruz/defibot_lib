// use the nom library to parse a command and generate the corresponding request
// commands look like :
//  !defi player_1 player_2 game score_1 score_2 -> gives a result id
//  !pending -> lists all pending requests
//  !pending player_name -> lists all pending requests for a specific player
//  !pending game -> lists all requests for a given game
//  !confirm result_id -> confirms the given pending request


// !defi
//
// Initiates a defi by one of its participants.
//
// # TODO
//
//  * the request must be initiated by one of its participants
//  * unless they have some kind of authorization ()
//
// # Outcomes
//
//  * Error: the DefiRequest creation gives back an error
//  * Success: the defi is stored in the persistence system
use nom::{alpha, alphanumeric, digit};

use std::str;
use std::str::FromStr;
use command::commands::DefiCommand;

named!(defi<&[u8], DefiCommand>,
       do_parse!(
           ws!(map_res!(tag!("!defi"), str::from_utf8)) >>
               players: nicks >>
               game: game >>
               scores: score >>
               (DefiCommand{
                   player_a: String::from(players.0),
                   player_b: String::from(players.1),
                   game: String::from(game),
                   score_a: scores.0,
                   score_b: scores.1
               })
       )
);

named!(game<&str>,
       ws!(map_res!(alphanumeric, str::from_utf8))
);

named!(nicks<(&str, &str)>,
       pair!(
           ws!(map_res!(alpha, str::from_utf8)),
           ws!(map_res!(alpha, str::from_utf8))
       )
);

named!(single_score<usize>,
       map_res!(
           map_res!(
               ws!(digit),
               str::from_utf8),
       FromStr::from_str)
);

named!(score<(usize, usize)>,
       pair!(
           single_score,
           single_score
       )
);


// !pending
//
// Lists all pending requests with their corresponding ids, can be given
// additional optional parameters to further filter down the list.
//
// # Outcomes
//
//  * Error: the requested game or player does not exist in the system
//  * Otherwise lists the pending results


// !confirm
//
// Confirmation of the result from the other player.
//
// # Outcomes
//
//  * Error: the id does not match to any pending result
//  * Error: the player does not have the right to confirm the result
//  * Ok: confirms the result, saves it in the persistence system
