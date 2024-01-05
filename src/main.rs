mod round;
mod cumulative_score;
mod hand;
mod dice_set;

use std::error::Error;
use std::process::ExitCode;
use crate::cumulative_score::CumulativeScore;
use crate::round::play_round;

fn main() -> Result<ExitCode, Box<dyn Error>> {

    // figure out the game configuration

    let players = ["Charlie".to_string(), "Maggie".to_string()];
    let mut score = CumulativeScore::new(players.clone(), 10_000);

    let mut players_iter = players.iter().cycle();

    loop {
        // check if someone has gone out
        if let Some(winner) = score.has_winner() {
            break winner;
        }

        // get the next player
        let Some(next_player) = players_iter.next() else {
            panic!("Player iterator cycle broken")
        };

        println!("{}'s turn:", next_player);

        // play for the player
        let round_score = play_round();

        // update that user's score
        score.update_user_score(next_player, round_score);
    };


    Ok(ExitCode::SUCCESS)
}
