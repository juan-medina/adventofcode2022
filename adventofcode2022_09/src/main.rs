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

use adventofcode2022_lib::utils::{read_file, Example, RunType};
use std::collections::HashMap;
mod move_step;
use move_step::Pos;

const NAME: &'static str = "Rope Bridge";
const OUTPUT: &'static str = "visited";
const FILE: &'static str = "rope_moves";

fn main() {
    Example::new(9, NAME, OUTPUT, FILE, solve_day_9).run_all();
}

fn solve_day_9(filename: &str, run_type: RunType) -> usize {
    let lines = read_file(filename);

    let start_pos = Pos { x: 0, y: 0 };

    let mut visits: HashMap<String, usize> = HashMap::new();
    *visits.entry(start_pos.to_key()).or_insert(0) += 1;

    let total_nodes = match run_type {
        RunType::Part1 => 2,
        RunType::Part2 => 10,
    };

    let mut nodes: Vec<Pos> = vec![start_pos;total_nodes];

    for line in lines {
        let (head_move, amount) = move_step::get_moves(&line);
        for _ in 1..amount + 1 {
            nodes[0].move_head(head_move);

            for i in 1..nodes.len() {
                let cur = &nodes[i];
                let prev = &nodes[i - 1];
                if !&cur.are_close(prev) {
                    nodes[i] = cur.move_close(prev);
                    if i == nodes.len() - 1 {
                        *visits.entry(nodes[i].to_key()).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    visits.len()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(9, NAME, OUTPUT, FILE, solve_day_9);
        assert_eq!(13, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(1, NAME, OUTPUT, FILE, solve_day_9);
        assert_eq!(1, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
