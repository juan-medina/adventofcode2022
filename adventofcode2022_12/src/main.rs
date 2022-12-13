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

use std::collections::VecDeque;
use adventofcode2022_lib::utils::{Example, read_file, RunType};

const NAME: &'static str = "Hill Climbing Algorithm";
const OUTPUT: &'static str = "steps";
const FILE: &'static str = "heightmap";

fn main() {
    Example::new(12, NAME, OUTPUT, FILE, solve_day_12).run_all();
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn zero() -> Self {
        Self::new(0, 0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct SearchStep {
    from: Node,
    num_step: usize,
}

impl SearchStep {
    fn new(from: Node, step: usize) -> Self {
        Self { from, num_step: step }
    }
}

static START_SYMBOL: char = 'S';
static END_SYMBOL: char = 'E';

static START_HEIGHT: usize = 'a' as usize - 1;
static END_HEIGHT: usize = 'z' as usize + 1;

static LOWEST_HEIGHT: usize = 'a' as usize;

fn solve_day_12(_filename: &str, _run_type: RunType) -> usize {
    let mut end_node = Node::zero();

    let mut x = 0usize;
    let mut y = 0usize;
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in read_file(_filename) {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            let value = if c == START_SYMBOL {
                START_HEIGHT
            } else if c == END_SYMBOL {
                end_node = Node::new(x, y);
                END_HEIGHT
            } else {
                c as usize
            };
            row.push(value);
            x += 1;
        }
        map.push(row);
        y += 1;
        x = 0;
    }

    let height = map.len();
    let width = map[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut find_queue = VecDeque::from([SearchStep::new(end_node, 0)]);
    let mut shorter_path = usize::MAX;

    let height_to_find = match _run_type {
        RunType::Part1 => {
            START_HEIGHT
        }
        RunType::Part2 => {
            LOWEST_HEIGHT
        }
    };

    while find_queue.len() > 0 {
        let search_step = find_queue.pop_front().unwrap();
        let search_node_height = map[search_step.from.y][search_step.from.x];
        if search_node_height == height_to_find {
            shorter_path = search_step.num_step;
            break;
        }

        let near_nodes = get_near_points(&search_step.from, width, height);
        for near in near_nodes {
            if map[near.y][near.x] >= search_node_height - 1 && !visited[near.y][near.x] {
                visited[near.y][near.x] = true;
                find_queue.push_back(SearchStep::new(near, search_step.num_step + 1));
            }
        }
    }

    shorter_path
}

fn get_near_points(n: &Node, width: usize, height: usize) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    if n.x > 0 {
        nodes.push(Node::new(n.x - 1, n.y));
    }
    if n.x < width - 1 {
        nodes.push(Node::new(n.x + 1, n.y));
    }
    if n.y > 0 {
        nodes.push(Node::new(n.x, n.y - 1));
    }
    if n.y < height - 1 {
        nodes.push(Node::new(n.x, n.y + 1));
    }

    nodes
}


#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(12, NAME, OUTPUT, FILE, solve_day_12);
        assert_eq!(31, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(12, NAME, OUTPUT, FILE, solve_day_12);
        assert_eq!(29, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
