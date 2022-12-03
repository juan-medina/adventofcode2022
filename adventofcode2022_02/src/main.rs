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
    println!("Day 2: Rock Paper Scissors");
    println!();
    solve_day_2_part_1();
    solve_day_2_part_2();
}

const EXAMPLE_FILE: &str = "data/strategy_example.dat";
const PUZZLE_FILE: &str = "data/strategy_puzzle.dat";

fn solve_day_2_part_1() {
    solve_day_2("part 1 [example]", EXAMPLE_FILE, false);
    solve_day_2("part 1 [puzzle]", PUZZLE_FILE, false);
}

fn solve_day_2_part_2() {
    solve_day_2("part 2 [example]", EXAMPLE_FILE, true);
    solve_day_2("part 2 [puzzle]", PUZZLE_FILE, true);
}

fn solve_day_2(label: &str, filename: &str, using_strategy: bool) {
    // total points
    let mut total_points = 0;

    //open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        // get the moves
        let (adversary_move, player_move) = get_moves(line.unwrap(), using_strategy);

        // calculate result
        let match_result = rules::check_winner(adversary_move, player_move);

        // accumulate points
        total_points += rules::get_points(player_move, match_result);
    }
    // print final points
    println!("{label} Total Points: {}", total_points);
}

// get the turn: adversary move, and strategy (win, loose, draw)
fn get_turn(line: String) -> (Move, Strategy) {
    let (left, right) = line.split_once(" ").unwrap();
    (
        Move::from_str(left).unwrap(),
        Strategy::from_str(right).unwrap(),
    )
}

fn get_moves(line: String, using_strategy: bool) -> (Move, Move) {
    if !using_strategy {
        let (left, right) = line.split_once(" ").unwrap();
        (
            Move::from_str(left).unwrap(),
            Move::from_str(right).unwrap(),
        )
    } else {
        // get turn
        let (adversary_move, strategy) = get_turn(line);
        // calculate player move
        let player_move = rules::generate_player_move(adversary_move, strategy);
        (
            adversary_move, player_move
        )
    }
}
