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

use adventofcode2022_lib::utils::{
    get_range_in, get_range_out, read_file, Example, FileType, RunType,
};

const NUM: &'static usize = &8;
const NAME: &'static str = "Treetop Tree House";
const OUTPUT: &'static [&'static str] = &["trees visible outside", "highest scenic score"];
const FILE: &'static str = "forest";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_8).run_all();
}

fn solve_day_8(filename: &str, run_type: RunType, _: FileType) -> u32 {
    let mut forest: Vec<Vec<u32>> = Vec::new();

    for lines in read_file(filename) {
        let mut tree_row: Vec<u32> = Vec::new();
        for tree in lines.chars() {
            tree_row.push(tree.to_digit(10).unwrap());
        }
        forest.push(tree_row);
    }

    match run_type {
        RunType::Part1 => hidden_trees(&mut forest),
        RunType::Part2 => max_view(&mut forest),
    }
}

fn hidden_trees(forest: &Vec<Vec<u32>>) -> u32 {
    let mut count: u32 = 0;

    let width = forest.len();
    let height = forest[0].len();

    for row in 0..width {
        for col in 0..height {
            if row < 1 || row > width - 2 || col < 1 || col > height - 2 {
                count += 1;
            } else {
                if is_hidden(forest, col, row) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn max_view(forest: &Vec<Vec<u32>>) -> u32 {
    let mut max_view: u32 = 0;

    let width = forest.len();
    let height = forest[0].len();

    for row in 0..width {
        for col in 0..height {
            if !(row < 1 || row > width - 2 || col < 1 || col > height - 2) {
                let current_view = views(forest, col, row);
                if current_view > max_view {
                    max_view = current_view;
                }
            }
        }
    }
    max_view
}

fn is_hidden(forest: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    !(is_hidden_horizontal(forest, col, row, true)
        && is_hidden_horizontal(forest, col, row, false)
        && is_hidden_vertical(forest, col, row, true)
        && is_hidden_vertical(forest, col, row, false))
}

fn is_hidden_horizontal(forest: &Vec<Vec<u32>>, col: usize, row: usize, from_left: bool) -> bool {
    let width = forest.len();
    let tree_height = forest[row][col];

    for i in get_range_in(0, col, width, !from_left) {
        if tree_height <= forest[row][i] {
            return true;
        }
    }

    return false;
}

fn is_hidden_vertical(forest: &Vec<Vec<u32>>, col: usize, row: usize, from_top: bool) -> bool {
    let height = forest[0].len();
    let tree_height = forest[row][col];

    for i in get_range_in(0, row, height, !from_top) {
        if tree_height <= forest[i][col] {
            return true;
        }
    }

    return false;
}

fn views(forest: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
    views_from_horizontal(forest, col, row, true)
        * views_from_horizontal(forest, col, row, false)
        * views_from_vertical(forest, col, row, true)
        * views_from_vertical(forest, col, row, false)
}

fn views_from_horizontal(forest: &Vec<Vec<u32>>, col: usize, row: usize, from_left: bool) -> u32 {
    let width = forest.len();
    let tree_height = forest[row][col];

    let mut viewed_trees = 0;

    for i in get_range_out(0, col, width, !from_left) {
        viewed_trees += 1;
        if forest[row][i] >= tree_height {
            break;
        }
    }

    return viewed_trees;
}

fn views_from_vertical(forest: &Vec<Vec<u32>>, col: usize, row: usize, from_top: bool) -> u32 {
    let height = forest[0].len();
    let tree_height = forest[row][col];

    let mut viewed_trees = 0;

    for i in get_range_out(0, row, height, !from_top) {
        viewed_trees += 1;
        if forest[i][col] >= tree_height {
            break;
        }
    }

    return viewed_trees;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_8);
        assert_eq!(21, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_8);
        assert_eq!(8, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
