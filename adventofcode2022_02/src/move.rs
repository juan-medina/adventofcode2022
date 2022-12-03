use std::str::FromStr;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissor),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Rock => write!(f, "Rock"),
            Move::Paper => write!(f, "Paper"),
            Move::Scissor => write!(f, "Scissor"),
        }
    }
}
