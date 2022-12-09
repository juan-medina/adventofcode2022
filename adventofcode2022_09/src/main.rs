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

mod move_step;

use crate::move_step::MoveStep;
use adventofcode2022_lib::utils::{read_file, Example, RunType};

const NAME: &'static str = "Rope Bridge";
const OUTPUT: &'static str = "visited";
const FILE: &'static str = "rope_moves";

fn main() {
    Example::new(9, NAME, OUTPUT, FILE, solve_day_9).run_all();
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

fn get_arena_size(lines: &Vec<String>) -> (usize, usize, usize, usize) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;

    for line in lines {
        let (head_move, amount) = move_step::get_moves(line);

        match head_move {
            MoveStep::Right => {
                x += amount as i32;
            }
            MoveStep::Left => {
                x -= amount as i32;
            }
            MoveStep::Up => {
                y += amount as i32;
            }
            MoveStep::Down => {
                y -= amount as i32;
            }
        }
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
    }

    let size_x = min_x.abs() + max_x;
    let size_y = min_y.abs() + max_y;

    let start_x = min_x.abs();
    let start_y = size_y - min_y.abs();

    (
        (size_x + 1) as usize,
        (size_y + 1) as usize,
        start_x as usize,
        start_y as usize,
    )
}

fn display_arena(arena: &Vec<Vec<usize>>, nodes: &Vec<Pos>) {
    let mut display: Vec<Vec<char>> = Vec::new();

    for row in arena {
        let mut arr: Vec<char> = Vec::new();
        for value in row {
            if value > &0 {
                arr.push('#');
            } else {
                arr.push('.');
            }
        }
        display.push(arr);
    }
    println!();

    for i in 0..nodes.len() {
        let node = nodes[i];
        let char;
        if i == 0 {
            char = 'H'
        } else if i == nodes.len() - 1 {
            char = 'T';
        } else {
            char = char::from_digit(i as u32, 10).unwrap();
        }
        display[node.y][node.x] = char;
    }

    for row in display {
        for char in row {
            print!("{}", char);
        }
        println!();
    }
    println!();
}

fn init_arena(width: usize, height: usize) -> Vec<Vec<usize>> {
    let mut arena: Vec<Vec<usize>> = Vec::new();
    for _ in 0..height {
        let mut row: Vec<usize> = Vec::new();
        for _ in 0..width {
            row.push(0);
        }
        arena.push(row);
    }
    arena
}

fn solve_day_9(filename: &str, run_type: RunType) -> usize {
    let lines = read_file(filename);

    let (width, height, start_x, start_y) = get_arena_size(&lines);
    let mut arena = init_arena(width, height);

    let start_pos = Pos {
        x: start_x,
        y: start_y,
    };
    arena[start_y][start_x] = 1;

    let mut nodes: Vec<Pos> = Vec::new();

    let total_nodes = match run_type {
        RunType::Part1 => 2,
        RunType::Part2 => 10,
    };

    for _ in 0..total_nodes {
        nodes.push(start_pos.clone())
    }

    for line in lines {
        let (head_move, amount) = move_step::get_moves(&line);

        for _ in 1..amount + 1 {
            nodes[0] = move_head(&nodes[0], head_move);

            for i in 1..nodes.len() {
                if !are_close(&nodes[i - 1], &nodes[i]) {
                    nodes[i] = move_close(&nodes[i - 1], &nodes[i]);
                    if i == nodes.len() - 1 {
                        arena[nodes[i].y][nodes[i].x] += 1;
                    }
                } else {}
            }
        }
    }
    count_visits(&arena)
}

fn are_close(pos1: &Pos, pos2: &Pos) -> bool {
    let min_x = pos2.x as i32 - 1;
    let max_x = pos2.x as i32 + 1;
    let min_y = pos2.y as i32 - 1;
    let max_y = pos2.y as i32 + 1;

    pos1.x as i32 >= min_x
        && pos1.x as i32 <= max_x
        && pos1.y as i32 >= min_y
        && pos1.y as i32 <= max_y
}

fn move_head(pos: &Pos, head_move: MoveStep) -> Pos {
    let mut head_pos = pos.clone();
    match head_move {
        MoveStep::Right => {
            head_pos.x += 1;
        }
        MoveStep::Left => {
            head_pos.x -= 1;
        }
        MoveStep::Up => {
            head_pos.y -= 1;
        }
        MoveStep::Down => {
            head_pos.y += 1;
        }
    }
    return head_pos;
}

fn move_close(pos1: &Pos, pos2: &Pos) -> Pos {
    let diff_x: i32 = (pos1.x as i32 - pos2.x as i32) as i32;
    let diff_y: i32 = (pos1.y as i32 - pos2.y as i32) as i32;

    Pos {
        x: (pos1.x as i32 - (diff_x / 2)) as usize,
        y: (pos1.y as i32 - (diff_y / 2)) as usize,
    }
}

fn count_visits(arena: &Vec<Vec<usize>>) -> usize {
    let mut total = 0;
    for row in arena {
        for state in row {
            if state > &0 {
                total += 1;
            }
        }
    }
    total
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
