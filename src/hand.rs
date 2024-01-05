pub mod dice_action;
pub mod dice_combination;
mod disp;

use std::collections::HashMap;
use crate::dice_set::{DiceSet};
use crate::hand::dice_action::{DiceAction, ScoreAction};
use crate::hand::dice_combination::DiceCombination;

#[derive(Default)]
pub struct Hand {
    /// which dice we have saved by doing this
    saved_dice: DiceSet,
    /// the combination we currently hold
    combos: Vec<DiceCombination>,
    /// maps single scoring dice combinations. The key is the die value and the value is the index
    single_scoring: HashMap<u8, usize>,
    /// multiple scoring dice combo map
    dice_multiples: HashMap<u8, usize>,
    /// how many scores have been had so far
    cumulative_score: u32,
    /// points guaranteed by rolling straights
    guaranteed_score: u32,
}

impl Hand {
    pub fn cumulative_score(&self) -> u32 {
        self.cumulative_score
    }

    /// set of dice that we have saved
    pub fn saved_dice(&self) -> &DiceSet {
        &self.saved_dice
    }

    /// Determine the different actions we can perform given the set of dice
    pub fn determine_actions(&self, dice: &DiceSet) -> Vec<DiceAction> {
        let mut actions = Vec::<DiceAction>::new();

        // first, check if we have pairs
        if let Some((one, two, three)) = dice.is_pairs() {
            actions.push(DiceAction {
                dice: dice.dice_values(),
                action: ScoreAction::KeepNew(DiceCombination::Pairs(one, two, three)),
            })
        }

        // next, check if there's any way we can combine our saved dice with these newly rolled dice
        let self_saved_dice = self.saved_dice().clone();
        let saved_dice_len = self.saved_dice().size();
        if dice.clone().union(self_saved_dice).is_straight() {
            let action = match saved_dice_len {
                0 => ScoreAction::KeepNew(DiceCombination::Straight { roll: 1 }),
                other => ScoreAction::AddTo(DiceCombination::Straight { roll: (1 + other) as u8 }),
            };

            // these dice make a straight
            actions.push(DiceAction {
                dice: dice.dice_values(),
                action,
            });
        }

        // next, check to see if we can add any dice to existing combinations
        for (multiple_die_value, idx) in &self.dice_multiples {
            if let Some((die_value, count)) = dice.get_die_count(*multiple_die_value) {
                let existing_combo = self.combos[*idx];
                actions.push(DiceAction {
                    dice: vec![die_value; count as usize],
                    action: ScoreAction::AddTo(existing_combo),
                })
            }
        }

        // check, if there is anything we can save from the roll alone
        let multiples_standalone_actions = dice.find_multiples();
        multiples_standalone_actions.into_iter()
            .map(|combo| DiceAction {
                dice: combo.involved_dice(),
                action: ScoreAction::KeepNew(combo),
            })
            .for_each(|action| actions.push(action));

        // finally, check for singles
        let singles_standalone_actions = dice.find_singles();
        for combo in singles_standalone_actions {
            let DiceCombination::Single { value, .. } = combo else {
                panic!()
            };

            // if we have one already, add to, otherwise, add new
            let action = if let Some(existing_combo) = self.single_scoring.get(&value).and_then(|idx| self.combos.get(*idx)) {
                DiceAction {
                    dice: combo.involved_dice(),
                    action: ScoreAction::AddTo(*existing_combo),
                }
            } else {
                DiceAction {
                    dice: combo.involved_dice(),
                    action: ScoreAction::KeepNew(combo),
                }
            };

            actions.push(action);
        }

        // if there are actions, the user can stay
        if !actions.is_empty() {
            let all_combos = actions.iter()
                .flat_map(|action| match &action.action {
                    ScoreAction::KeepNew(combo) => vec![*combo],
                    ScoreAction::AddTo(combo) => vec![*combo],
                    ScoreAction::Stay(combos) => combos.clone()
                })
                .collect::<Vec<_>>();

            let involved_dice = all_combos.iter()
                .flat_map(|combo| combo.involved_dice())
                .collect::<Vec<_>>();

            actions.push(DiceAction {
                dice: involved_dice,
                action: ScoreAction::Stay(all_combos),
            });
        }

        actions
    }

    fn upsert_combo(&mut self, combo: DiceCombination) {
        match combo {
            DiceCombination::Single { value, max_count } => {
                let idx = *self.single_scoring.get(&value).unwrap();
                let DiceCombination::Single { max_count: existing_max_count, .. } = self.combos.get_mut(idx).unwrap() else {
                    panic!()
                };

                // update the max count
                *existing_max_count += max_count;
            }
            DiceCombination::Multiple { value, quantity} => {
                let idx = *self.dice_multiples.get(&value).unwrap();
                let DiceCombination::Multiple { quantity: existing_qty, .. } = self.combos.get_mut(idx).unwrap() else {
                    panic!()
                };

                // update the quantity
                *existing_qty += quantity;
            }
            _ => {}
        }
    }

    /// true if we should keep going
    pub fn perform_action(&mut self, action: DiceAction) -> bool {
        // add any guaranteed score
        if let Some(score) = action.has_guaranteed_score() {
            self.guaranteed_score += score;
        }

        // add the combo
        match action.action {
            ScoreAction::KeepNew(combo) => {
                let combo_idx = self.combos.len();
                self.combos.push(combo);

                // update the appropriate map with the index
                match combo {
                    DiceCombination::Single { value, .. } => {
                        self.single_scoring.insert(value, combo_idx);
                    }
                    DiceCombination::Multiple { value, .. } => {
                        self.dice_multiples.insert(value, combo_idx);
                    }
                    _ => {}
                }
                true
            }
            ScoreAction::AddTo(combo) => {
                self.upsert_combo(combo);
                true
            }
            ScoreAction::Stay(combos) => {
                for combo in combos {
                    self.upsert_combo(combo)
                }

                false
            }
        }
    }

    /// score the combos currently in your hand
    pub fn score_combos(&self) -> u32 {
        self.combos.iter()
            .map(|combo| combo.score())
            .sum()
    }

    pub fn accumulate_score(&mut self) {
        let combo_score = self.score_combos();
        self.cumulative_score += combo_score;
        self.combos.clear();
        self.single_scoring.clear();
        self.dice_multiples.clear();
    }
}
