use std::error::Error;
use std::io::{BufRead, stdin, stdout, Write};
use crate::dice_set::DiceSet;
use crate::hand::dice_action::{DiceAction};
use crate::hand::dice_combination::DiceCombination;
use crate::hand::Hand;

pub fn play_round() -> u32 {
    // the current hand for this user
    let mut hand = Hand::default();

    let mut round_num = 1;
    let mut roll_count = 6;

    let has_score = loop {
        // roll some dice
        let rolled_dice = DiceSet::rand(roll_count);
        println!("Roll {}:", round_num);
        println!("{}", rolled_dice);

        // show the user what they can save from their roll
        let actions = hand.determine_actions(&rolled_dice);
        if actions.is_empty() {
            println!("Nothing scored. You lost all your points");
            break false;
        }

        // figure out which scores the user is going to save
        println!("Possible actions:");
        let selected_action = select_dice_action(actions);
        println!("You selected: {}", selected_action);
        
        // update how many dice we are going to roll next time
        roll_count -= selected_action.dice.len();

        // update the dice
        let keep_going = hand.perform_action(selected_action);
        println!("Your hand:\n{}", hand);
        if !keep_going {
            println!("You stopped");
            break true;
        }

        // if we are out of dice to roll, we have to roll everything again
        if roll_count == 0 {
            hand.accumulate_score();
            roll_count = 6;
        }

        round_num += 1;
    };

    if has_score {
        hand.cumulative_score()
    } else {
        0
    }
}

fn select_dice_action(available_actions: Vec<DiceAction>) -> DiceAction {
    let selected_index = loop {
        for (idx, action) in available_actions.iter().enumerate() {
            println!("{}: {}", idx, action)
        }

        print!("Select action to take: ");
        stdout().flush().expect("Flushing should not fail");
        let selected_index = read_number(&mut stdin().lock());

        let Ok(selected_index) = selected_index else {
            println!("You must select at least one score");
            continue;
        };

        if selected_index >= available_actions.as_slice().len() {
            println!("{} is not a valid index", selected_index);
            continue;
        }

        break selected_index;
    };

    // TODO everything from here on is super, super ugly. It needs to be cleaned up
    let selected_action = available_actions.into_iter().nth(selected_index).unwrap();
    let updated_action = match selected_action.action.dice_combo() {
        DiceCombination::Single { max_count, value } => loop {
            let max_count = max_count as usize;
            print!("Select amount you want to take (max {}): ", max_count);
            stdout().flush().expect("Flush should not fail");
            let Ok(count) = read_number(&mut stdin().lock()) else {
                continue;
            };
            
            if count > max_count {
                println!("{} is too many", count);
                continue;
            }
            
            break DiceCombination::Single { value, max_count: count as u8 }
        }
        other => other,
    };
    
    let score_action = selected_action.action.with_combo(updated_action);
    DiceAction {
        dice: selected_action.dice,
        action: score_action,
    }
}

fn read_number<InputT: BufRead>(input: &mut InputT) -> Result<usize, Box<dyn Error>> {
    let mut line = String::new();
    input.read_line(&mut line)?;
    line.trim().parse::<usize>()
        .map_err(|err| err.into())
}
