use std::fmt::{Display, Formatter, write};
use crate::hand::DiceCombination;

pub enum ScoreAction {
    /// keep a new thing
    KeepNew(DiceCombination),
    /// add dice to an existing combination in the hand
    AddTo(DiceCombination),
    /// stop rolling all together, but keep these combinations
    Stay(Vec<DiceCombination>),
}

impl ScoreAction {
    pub fn dice_combo(&self) -> DiceCombination {
        match self {
            ScoreAction::KeepNew(combo) => *combo,
            ScoreAction::AddTo(combo) => *combo,
            ScoreAction::Stay(combos) => *combos.first().unwrap(),
        }
    }

    pub fn with_combo(self, new_combo: DiceCombination) -> Self {
        match self {
            ScoreAction::KeepNew(_) => Self::KeepNew(new_combo),
            ScoreAction::AddTo(_) => Self::AddTo(new_combo),
            ScoreAction::Stay(mut combos) => {
                combos.push(new_combo);
                Self::Stay(combos)
            }
        }
    }
}

pub struct DiceAction {
    /// the dice to keep
    pub dice: Vec<u8>,
    /// the action the user is performing by taking these dice
    pub action: ScoreAction,
}

impl DiceAction {
    pub fn has_guaranteed_score(&self) -> Option<u32> {
        match self.action {
            ScoreAction::KeepNew(action) => if action.is_straight_roll() {
                Some(action.score())
            } else {
                None
            }
            ScoreAction::AddTo(action) => if action.is_straight_roll() {
                Some(action.score())
            } else {
                None
            }
            _ => None
        }
    }
}

impl Display for DiceAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dice_str = self.dice.iter()
            .map(|die| format!("[{}]", die))
            .collect::<Vec<_>>()
            .join("");

        write!(f, "save {} to {}", dice_str, self.action)
    }
}

impl Display for ScoreAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ScoreAction::KeepNew(action) => write!(f, "form {}", action),
            ScoreAction::AddTo(action) => write!(f, "add to {}", action),
            ScoreAction::Stay(actions) => {
                let actions_str = actions.iter()
                    .map(|action| format!("{}", action))
                    .collect::<Vec<_>>()
                    .join(",");

                write!(f, "stay with {}", actions_str)
            },
        }
    }
}
