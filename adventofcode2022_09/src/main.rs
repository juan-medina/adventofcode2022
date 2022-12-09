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

extern crate core;

mod move_step;

use std::fmt;
use adventofcode2022_lib::utils::{Example, read_file, RunType};
use crate::move_step::MoveStep;

const NAME: &'static str = "Rope Bridge";
const OUTPUT: &'static str = "visited";
const FILE: &'static str = "rope_moves";

fn main() {
    Example::new(9, NAME, OUTPUT, FILE, solve_day_9).run_all();
}

#[derive(Clone, Copy)]
struct ArenaState {
    has_head: bool,
    has_tail: bool,
    tail_visit: usize,
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl fmt::Display for ArenaState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.has_head {
            write!(f, "H")
        } else if self.has_tail {
            write!(f, "T")
        } else if self.tail_visit > 0 {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
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
            MoveStep::Right => { x += amount as i32; }
            MoveStep::Left => { x -= amount as i32; }
            MoveStep::Up => { y += amount as i32; }
            MoveStep::Down => { y -= amount as i32; }
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

    ((size_x + 1) as usize, (size_y + 1) as usize, start_x as usize, start_y as usize)
}

fn display_arena(arena: &Vec<Vec<ArenaState>>) {
    for row in arena {
        for char in row {
            print!("{}", char);
        }
        println!();
    }
    println!();
}

fn init_arena(width: usize, height: usize) -> Vec<Vec<ArenaState>> {
    let mut arena: Vec<Vec<ArenaState>> = Vec::new();
    for _ in 0..height {
        let mut row: Vec<ArenaState> = Vec::new();
        for _ in 0..width {
            row.push(ArenaState { has_head: false, has_tail: false, tail_visit: 0 });
        }
        arena.push(row);
    }
    arena
}

fn solve_day_9(filename: &str, _run_type: RunType) -> usize {
    let lines = read_file(filename);

    let (width, height, start_x, start_y) = get_arena_size(&lines);
    let mut arena = init_arena(width, height);

    let mut head_pos = Pos { x: start_x, y: start_y };
    let mut tail_pos = head_pos.clone();

    arena[head_pos.y][head_pos.x] = ArenaState { has_head: true, has_tail: true, tail_visit: 1 };

    for line in lines {
        let (head_move, amount) = move_step::get_moves(&line);

        for _ in 1..amount + 1 {
            let mut prev_pos = head_pos.clone();

            match head_move {
                MoveStep::Right => { head_pos.x += 1; }
                MoveStep::Left => { head_pos.x -= 1; }
                MoveStep::Up => { head_pos.y -= 1; }
                MoveStep::Down => { head_pos.y += 1; }
            }
            arena[prev_pos.y][prev_pos.x].has_head = false;
            arena[head_pos.y][head_pos.x].has_head = true;

            if !has_head_near(&arena, &tail_pos) {
                prev_pos = tail_pos.clone();
                tail_pos = head_pos.clone();
                match head_move {
                    MoveStep::Right => {
                        tail_pos.x = head_pos.x - 1;
                    }
                    MoveStep::Left => {
                        tail_pos.x = head_pos.x + 1;
                    }
                    MoveStep::Up => {
                        tail_pos.y = head_pos.y + 1;
                    }
                    MoveStep::Down => {
                        tail_pos.y = head_pos.y - 1;
                    }
                }
                arena[prev_pos.y][prev_pos.x].has_tail = false;
                arena[tail_pos.y][tail_pos.x].has_tail = true;
                arena[tail_pos.y][tail_pos.x].tail_visit += 1;
            }
        }
    }
    display_arena(&arena);
    count_visits(&arena)
}

fn has_head_near(arena: &Vec<Vec<ArenaState>>, tail: &Pos) -> bool {
    let mut min_x = tail.x as i32 - 1;
    let mut max_x = tail.x as i32 + 1;
    let mut min_y = tail.y as i32 - 1;
    let mut max_y = tail.y as i32 + 1;

    let height = arena.len() as i32;
    let width = arena[0].len() as i32;

    if min_x < 0 {
        min_x = 0;
    }
    if max_x > width - 1 {
        max_x = width - 1;
    }
    if min_y < 0 {
        min_y = 0;
    }
    if max_y > height - 1 {
        max_y = height - 1;
    }
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if arena[y as usize][x as usize].has_head {
                return true;
            }
        }
    }

    false
}

fn count_visits(arena: &Vec<Vec<ArenaState>>) -> usize {
    let mut total = 0;
    for row in arena {
        for state in row {
            if state.tail_visit > 0 {
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
        let example = Example::new(9, NAME, OUTPUT, FILE, solve_day_9);
        assert_eq!(13, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
