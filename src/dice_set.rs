use std::cmp::{max, min};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use rand::distributions::{Distribution, Uniform};
use crate::hand::dice_combination::DiceCombination;

#[derive(Clone, Default)]
pub struct DiceSet {
    /// die value frequency
    freq: HashMap<u8, u8>,
    /// total number of dice in the set, memoized
    total: usize,
}

impl DiceSet {
    pub fn new() -> Self {
        Self {
            freq: HashMap::new(),
            total: 0,
        }
    }

    pub fn rand(count: usize) -> Self {
        let mut rng = &mut rand::thread_rng();
        let dice = Uniform::new_inclusive(1u8, 6u8).sample_iter(&mut rng)
            .take(count)
            .collect::<Vec<u8>>();

        Self::from(dice.as_slice())
    }

    fn with_capacity(cap: usize) -> Self {
        Self {
            freq: HashMap::with_capacity(cap),
            total: 0
        }
    }

    pub fn add_amount(&mut self, die: u8, amount: u8) {
        if self.freq.contains_key(&die) {
            self.freq.get_mut(&die).unwrap().add_assign(amount);
        } else {
            self.freq.insert(die, amount);
        }

        self.total += amount as usize;
    }

    pub fn add(&mut self, die: u8) {
        self.add_amount(die, 1)
    }

    pub fn remove_amount(&mut self, die: u8, amount: u8) {
        if !self.freq.contains_key(&die) {
            return;
        }

        let (die_value, mut freq) = self.freq.remove_entry(&die).unwrap();
        // decrease the total
        self.total -= min(amount as usize, freq as usize);

        freq = max(0u8, freq - amount);
        if freq > 0 {
            self.freq.insert(die_value, freq);
        }
    }

    pub fn remove_all(&mut self, die: u8) {
        self.freq.remove(&die);
    }

    /// finds any dice that have multiples. Returns a list of dice values that contain multiples
    pub fn take_multiples(self) -> Vec<u8> {
        self.freq.iter()
            .filter_map(|(die_value, freq)| if *freq >= 3 { Some(*die_value) } else { None })
            .collect()
    }

    /// unions with dice set with another
    pub fn union(mut self, other: Self) -> Self {
        for (die_value, freq) in other.freq {
            self.add_amount(die_value, freq)
        }

        self
    }

    pub fn is_straight(&self) -> bool {
        // there are 6 total dice and each one occurs exactly once
        self.total == 6 && self.freq.len() == 6
    }

    pub fn is_pairs(&self) -> Option<(u8, u8, u8)> {
        let pairs = self.freq.iter()
            .filter(|(_, freq)| **freq == 2)
            .map(|(die, _)| *die)
            .collect::<Vec<_>>();

        if pairs.len() == 3 {
            Some((*pairs.get(0).unwrap(), *pairs.get(1).unwrap(), *pairs.get(2).unwrap()))
        } else {
            None
        }
    }

    pub fn dice_values(&self) -> Vec<u8> {
        let mut values = Vec::new();
        for (value, freq) in &self.freq {
            for _ in 0..*freq {
                values.push(*value);
            }
        }

        values
    }

    pub fn is_empty(&self) -> bool {
        self.freq.is_empty()
    }

    pub fn size(&self) -> usize {
        self.total
    }

    pub fn get_die_count(&self, die: u8) -> Option<(u8, u8)> {
        self.freq.get_key_value(&die).map(|(value, freq)| (*value, *freq))
    }

    pub fn find_multiples(&self) -> Vec<DiceCombination> {
        self.freq.iter()
            .filter_map(|(value, freq)| {
                if *freq >= 3 {
                    Some(DiceCombination::Multiple {
                        value: *value,
                        quantity: *freq - 2,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn find_singles(&self) -> Vec<DiceCombination> {
        self.freq.iter()
            .filter_map(|(value, count)| match *value {
                1|5 => Some(DiceCombination::Single {
                    value: *value,
                    max_count: *count,
                }),
                _ => None
            })
            .collect()
    }
    
    pub fn has_die_value(&self, die: u8) -> bool {
        self.freq.contains_key(&die)
    }
}

impl From<&[u8]> for DiceSet {
    fn from(value: &[u8]) -> Self {
        let mut dice_set = DiceSet::with_capacity(value.len());
        for die_value in value {
            dice_set.add(*die_value);
        }

        dice_set
    }
}

impl Display for DiceSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (die_value, freq) in &self.freq {
            for _ in 0..*freq {
                write!(f, "[{}]", die_value)?;
            }
        }

        Ok(())
    }
}
