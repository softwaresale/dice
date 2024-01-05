use std::collections::HashMap;
use std::ops::AddAssign;

pub struct CumulativeScore {
    /// each player name and their score
    scores: HashMap<String, u32>,
    /// what the players are playing to
    limit: u32,
}

impl CumulativeScore {
    pub fn new<VecT: IntoIterator<Item=String>>(players: VecT, limit: u32) -> Self {
        let scores = players.into_iter()
            .map(|player| (player, 0u32))
            .collect::<HashMap<String, u32>>();

        Self {
            scores,
            limit
        }
    }

    pub fn has_winner(&self) -> Option<&String> {
        self.scores.iter()
            .find_map(|(player, score)| if *score >= self.limit { Some(player) } else { None })
    }

    pub fn update_user_score(&mut self, user: &str, points: u32) {
        self.scores.get_mut(user).unwrap().add_assign(points);
    }
}
