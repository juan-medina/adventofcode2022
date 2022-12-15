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

use adventofcode2022_lib::utils::{Example, FileType, RunType};
use std::collections::VecDeque;

mod map;
mod node;
mod step;

const NUM: &'static usize = &12;
const NAME: &'static str = "Hill Climbing Algorithm";
const OUTPUT: &'static [&'static str] = &["steps to E", "steps to a"];
const FILE: &'static str = "heightmap";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_12).run_all();
}

fn solve_day_12(filename: &str, run_type: RunType, _: FileType) -> usize {
    let mut shorter_path = usize::MAX;

    let (first_step, map) = map::get_map(filename);

    let height = map.len();
    let width = map[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut navigation_queue = VecDeque::from([first_step]);

    let height_to_navigate_to = match run_type {
        RunType::Part1 => map::START_HEIGHT,
        RunType::Part2 => map::LOWEST_HEIGHT,
    };

    while navigation_queue.len() > 0 {
        let step = navigation_queue.pop_front().unwrap();
        let node_height = map[step.from.y][step.from.x];
        if node_height == height_to_navigate_to {
            shorter_path = step.num_step;
            break;
        }

        for near in step.from.get_near_nodes(width, height) {
            if map[near.y][near.x] >= node_height - 1 && !visited[near.y][near.x] {
                visited[near.y][near.x] = true;
                navigation_queue.push_back(near.step(step.num_step + 1));
            }
        }
    }

    shorter_path
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_12);
        assert_eq!(31, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_12);
        assert_eq!(29, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
