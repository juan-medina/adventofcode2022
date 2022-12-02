use std::{
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Moves {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Moves {
    type Err = ();

    fn from_str(input: &str) -> Result<Moves, Self::Err> {
        match input {
            "A" => Ok(Moves::Rock),
            "B" => Ok(Moves::Paper),
            "C" => Ok(Moves::Scissor),
            "X" => Ok(Moves::Rock),
            "Y" => Ok(Moves::Paper),
            "Z" => Ok(Moves::Scissor),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Moves::Rock => write!(f, "Rock"),
            Moves::Paper => write!(f, "Paper"),
            Moves::Scissor => write!(f, "Scissor"),
        }
    }
}

enum MatchResult {
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

fn check_match(adversary: &Moves, player: &Moves) -> MatchResult {
    return match player {
        Moves::Rock => match adversary {
            Moves::Rock => MatchResult::Draw,
            Moves::Paper => MatchResult::PlayerLoose,
            Moves::Scissor => MatchResult::PlayerWins,
        },
        Moves::Paper => match adversary {
            Moves::Rock => MatchResult::PlayerWins,
            Moves::Paper => MatchResult::Draw,
            Moves::Scissor => MatchResult::PlayerLoose,
        },
        Moves::Scissor => match adversary {
            Moves::Rock => MatchResult::PlayerLoose,
            Moves::Paper => MatchResult::PlayerWins,
            Moves::Scissor => MatchResult::Draw,
        },
    };
}

fn points_per_move(player: &Moves) -> i32 {
    return match player {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissor => 3,
    };
}

fn points_per_result(result: &MatchResult) -> i32 {
    return match result {
        MatchResult::PlayerWins => 6,
        MatchResult::PlayerLoose => 0,
        MatchResult::Draw => 3,
    };
}

fn main() {
    // total points
    let mut total_points = 0;

    //open the file
    let filename = "data/strategy.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (a, b) = line.split_at(1);

        // get the moves
        let adversary_move = Moves::from_str(a).unwrap();
        let player_move = Moves::from_str(b.trim()).unwrap();

        // calculate result
        let result = check_match(&adversary_move, &player_move);

        // get the points
        let player_points = points_per_move(&player_move) + points_per_result(&result);

        total_points += player_points;

        // print stats
        println!(
            "Adversary: {} - Player: {}, {}! Match Points: {}",
            adversary_move, player_move, result, player_points
        );
    }
    println!("Total Points: {}", total_points);
}