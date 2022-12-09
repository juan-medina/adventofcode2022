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

use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy)]
pub enum MoveStep {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for MoveStep {
    type Err = ();

    fn from_str(input: &str) -> Result<MoveStep, Self::Err> {
        match input {
            "R" => Ok(MoveStep::Right),
            "L" => Ok(MoveStep::Left),
            "U" => Ok(MoveStep::Up),
            "D" => Ok(MoveStep::Down),
            _ => Err(()),
        }
    }
}

pub fn get_moves(line: &String) -> (MoveStep, usize) {
    let (left, right) = line.split_once(" ").unwrap();
    (
        MoveStep::from_str(left).unwrap(),
        right.parse::<usize>().unwrap(),
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn to_key(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }

    pub fn move_head(&mut self, head_move: MoveStep) {
        match head_move {
            MoveStep::Right => {
                self.x += 1;
            }
            MoveStep::Left => {
                self.x -= 1;
            }
            MoveStep::Up => {
                self.y -= 1;
            }
            MoveStep::Down => {
                self.y += 1;
            }
        }
    }

    pub fn are_close(&self, pos: &Pos) -> bool {
        pos.x >= (self.x - 1)
            && pos.x <= (self.x + 1)
            && pos.y >= (self.y - 1)
            && pos.y <= (self.y + 1)
    }

    pub fn move_close(&self, pos1: &Pos) -> Pos {
        Pos {
            x: (pos1.x - ((pos1.x - self.x) / 2)),
            y: (pos1.y - ((pos1.y - self.y) / 2)),
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.x, self.y)
    }
}
