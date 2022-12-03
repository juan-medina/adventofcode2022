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

use crate::match_result::MatchResult;
use crate::r#move::Move;
use crate::strategy::Strategy;

// generate a player move base on the adversary move and strategy (player win, player loose, player draw)
pub fn generate_player_move(adversary: Move, strategy: Strategy) -> Move {
    match adversary {
        Move::Rock => match strategy {
            Strategy::PlayerMustWin => Move::Paper,
            Strategy::PlayerMustLoose => Move::Scissor,
            Strategy::PlayerMustDraw => Move::Rock,
        },
        Move::Paper => match strategy {
            Strategy::PlayerMustWin => Move::Scissor,
            Strategy::PlayerMustLoose => Move::Rock,
            Strategy::PlayerMustDraw => Move::Paper,
        },
        Move::Scissor => match strategy {
            Strategy::PlayerMustWin => Move::Rock,
            Strategy::PlayerMustLoose => Move::Paper,
            Strategy::PlayerMustDraw => Move::Scissor,
        },
    }
}

// check who wins a match between player and adversary
pub fn check_winner(adversary: Move, player: Move) -> MatchResult {
    match player {
        Move::Rock => match adversary {
            Move::Rock => MatchResult::Draw,
            Move::Paper => MatchResult::PlayerLoose,
            Move::Scissor => MatchResult::PlayerWins,
        },
        Move::Paper => match adversary {
            Move::Rock => MatchResult::PlayerWins,
            Move::Paper => MatchResult::Draw,
            Move::Scissor => MatchResult::PlayerLoose,
        },
        Move::Scissor => match adversary {
            Move::Rock => MatchResult::PlayerLoose,
            Move::Paper => MatchResult::PlayerWins,
            Move::Scissor => MatchResult::Draw,
        },
    }
}

// points per move
fn points_per_move(player: Move) -> i32 {
    match player {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    }
}

// points per match result
fn points_per_match_result(result: MatchResult) -> i32 {
    match result {
        MatchResult::PlayerWins => 6,
        MatchResult::PlayerLoose => 0,
        MatchResult::Draw => 3,
    }
}

// get the points from the move and the result
pub fn get_points(player_move: Move, match_result: MatchResult) -> i32 {
    points_per_move(player_move) + points_per_match_result(match_result)
}
