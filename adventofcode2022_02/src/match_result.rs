use std::fmt;

#[derive(Clone, Copy)]
pub enum MatchResult {
    PlayerWins,
    PlayerLoose,
    Draw,
}

impl fmt::Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchResult::PlayerWins => write!(f, "Player wins"),
            MatchResult::PlayerLoose => write!(f, "Player loose"),
            MatchResult::Draw => write!(f, "Draw"),
        }
    }
}
