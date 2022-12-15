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

use adventofcode2022_lib::utils;
use std::str::FromStr;

use adventofcode2022_lib::utils::{Example, FileType, RunType};

use r#move::Move;
use strategy::Strategy;

mod match_result;
mod r#move;
mod rules;
mod strategy;

const NUM: &'static usize = &2;
const NAME: &'static str = "Rock Paper Scissors";
const OUTPUT: &'static [&'static str] = &["score", "score"];
const FILE: &'static str = "strategy";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_2).run_all();
}

fn solve_day_2(filename: &str, run_type: RunType, _: FileType) -> u32 {
    // total points
    let mut total_points: u32 = 0;

    // read the file line by line
    for line in utils::read_file(filename) {
        // get the moves
        let (adversary_move, player_move) = get_moves(line, run_type == RunType::Part2);

        // calculate result
        let match_result = rules::check_winner(adversary_move, player_move);

        // accumulate points
        total_points += rules::get_points(player_move, match_result);
    }
    total_points
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
        (adversary_move, player_move)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_2);
        assert_eq!(15, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_2);
        assert_eq!(12, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
