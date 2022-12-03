use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum Strategy {
    PlayerMustWin,
    PlayerMustLoose,
    PlayerMustDraw,
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(input: &str) -> Result<Strategy, Self::Err> {
        match input {
            "X" => Ok(Strategy::PlayerMustLoose),
            "Y" => Ok(Strategy::PlayerMustDraw),
            "Z" => Ok(Strategy::PlayerMustWin),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Strategy::PlayerMustWin => write!(f, "must win"),
            Strategy::PlayerMustLoose => write!(f, "must loose"),
            Strategy::PlayerMustDraw => write!(f, "must draw"),
        }
    }
}
