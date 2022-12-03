/***
Copyright (c) 2022 Juan Medina

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
***/

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
