use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use r#move::Move;
use strategy::Strategy;

mod strategy;
mod r#move;
mod match_result;
mod rules;

fn main() {
    // total points
    let mut total_points = 0;

    //open the file
    let filename = "data/strategy.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        // get turn
        let (adversary_move, strategy) = get_turn(line.unwrap());

        // calculate player move
        let player_move = rules::generate_player_move(adversary_move, strategy);

        // calculate result
        let match_result = rules::check_winner(adversary_move, player_move);

        // get the points
        let player_points = rules::get_points(player_move, match_result);

        // accumulate points
        total_points += rules::get_points(player_move, match_result);

        // print stats
        println!(
            "Adversary: {} - Strategy: Player {} - Player: {}, {}! Match Points: {}",
            adversary_move, strategy, player_move, match_result, player_points
        );
    }
    // print final points
    println!("Total Points: {}", total_points);
}

// get the turn: adversary move, and strategy (win, loose, draw)
fn get_turn(line: String) -> (Move, Strategy) {
    let (left, right) = line.split_once(" ").unwrap();
    (
        Move::from_str(left).unwrap(),
        Strategy::from_str(right).unwrap(),
    )
}
