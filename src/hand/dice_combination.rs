use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum DiceCombination {
    /// A single die that has points
    Single {
        /// what the dice number is
        value: u8,
        /// the max number of this single you can save
        max_count: u8,
    },
    /// a three of a kind or more
    Multiple {
        value: u8,
        /// how many dice are saved
        quantity: u8,
    },
    /// A straight, from whatever roll
    Straight {
        /// which roll it happened on
        roll: u8,
    },
    /// Pairs
    Pairs(u8, u8, u8),
}

impl DiceCombination {
    pub fn score(&self) -> u32 {
        match self {
            DiceCombination::Single { value, max_count } => match *value {
                1 => 100u32 * (*max_count as u32),
                5 => 50u32 * (*max_count as u32),
                other => panic!("{} is not a valid single die configuration", other),
            }
            DiceCombination::Multiple { value, quantity } => {
                let value = if *value == 1 {
                    // corner case: multiple 1s is 1k each
                    1000
                } else {
                    (*value as u32) * 100
                };

                let qty = *quantity as u32;

                value * qty
            },
            DiceCombination::Straight { roll } => match *roll {
                1 => 1500,
                2 => 1000,
                3 => 500,
                _ => unreachable!()
            }
            DiceCombination::Pairs(_, _, _) => 1000,
        }
    }

    pub fn involved_dice(&self) -> Vec<u8> {
        match self {
            DiceCombination::Single { value, max_count } => vec![*value; *max_count as usize],
            DiceCombination::Multiple { value, quantity } => vec![*value; (*quantity + 2) as usize],
            DiceCombination::Straight { .. } => (1..=6).collect(),
            DiceCombination::Pairs(first, second, third) => vec![*first, *first, *second, *second, *third, *third],
        }
    }
    
    pub fn is_straight_roll(&self) -> bool {
        match self {
            DiceCombination::Straight { .. } => true,
            _ => false,
        }
    }
}

impl Display for DiceCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceCombination::Single { value, max_count } => write!(f, "{} {}s", max_count, value),
            DiceCombination::Multiple { value, quantity } => write!(f, "combo of {} {}s", *quantity + 2, value),
            DiceCombination::Straight { .. } => write!(f, "straight"),
            DiceCombination::Pairs(_, _, _) => write!(f, "pairs"),
        }
    }
}
