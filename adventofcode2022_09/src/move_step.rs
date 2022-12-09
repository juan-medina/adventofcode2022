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
