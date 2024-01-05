use std::fmt::{Display, Formatter};
use crate::hand::Hand;

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Cumulative score: {} ({} guaranteed)", self.cumulative_score, self.guaranteed_score)?;
        writeln!(f, "Saved combos: {} points", self.score_combos())?;
        for combo in &self.combos {
            writeln!(f, "{}", combo)?;
        }
        Ok(())
    }
}
