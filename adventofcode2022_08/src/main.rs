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

const NAME: &'static str = "Treetop Tree House";
const OUTPUT: &'static str = "trees";
const FILE: &'static str = "forest";

fn main() {
    Example::new(8, NAME, OUTPUT, FILE, solve_day_8).run_all();
}

fn solve_day_8(filename: &str, run_type: RunType) -> u32 {
    let mut forest: Vec<Vec<u32>> = Vec::new();

    for lines in read_file(filename) {
        let mut tree_row: Vec<u32> = Vec::new();
        for tree in lines.chars() {
            tree_row.push(tree.to_digit(10).unwrap());
        }
        forest.push(tree_row);
    }

    match run_type {
        RunType::Part1 => {
            hidden_trees(&mut forest)
        }
        RunType::Part2 => {
            max_view(&mut forest)
        }
    }
}

fn max_view(forest: &Vec<Vec<u32>>) -> u32 {
    let mut max_view: u32 = 0;

    let width = forest.len();
    let height = forest[0].len();

    for row in 0..width {
        for col in 0..height {
            if !(row < 1 || row > width - 2 || col < 1 || col > height - 2) {
                let tree_height = forest[row][col];

                let mut current_view = 1;

                // check from left
                let mut viewed_trees = 0;
                for i in (0..col).rev() {
                    viewed_trees += 1;
                    if forest[row][i] >= tree_height {
                        break;
                    }
                }
                current_view *= viewed_trees;

                // check from right
                viewed_trees = 0;
                for i in col + 1..width {
                    viewed_trees += 1;
                    if forest[row][i] >= tree_height {
                        break;
                    }
                }
                current_view *= viewed_trees;

                // check from top
                viewed_trees = 0;
                for j in (0..row).rev() {
                    viewed_trees += 1;
                    if forest[j][col] >= tree_height {
                        break;
                    }
                }
                current_view *= viewed_trees;

                // check from bottom
                viewed_trees = 0;
                for j in row + 1..height {
                    viewed_trees += 1;
                    if forest[j][col] >= tree_height {
                        break;
                    }
                }
                current_view *= viewed_trees;

                if current_view > max_view {
                    max_view = current_view;
                }
            }
        }
    }
    max_view
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
                let tree_height = forest[row][col];

                // check from left
                let mut hidden = false;
                for i in 0..col {
                    if tree_height <= forest[row][i] {
                        hidden = true;
                        break;
                    }
                }

                if hidden {
                    // check from right
                    hidden = false;
                    for i in (col + 1..width).rev() {
                        if tree_height <= forest[row][i] {
                            hidden = true;
                            break;
                        }
                    }
                }
                if hidden {
                    // check from top
                    hidden = false;
                    for j in 0..row {
                        if tree_height <= forest[j][col] {
                            hidden = true;
                            break;
                        }
                    }
                }
                if hidden {
                    // check from bottom
                    hidden = false;
                    for j in (row + 1..height).rev() {
                        if tree_height <= forest[j][col] {
                            hidden = true;
                            break;
                        }
                    }
                }
                if !hidden {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(8, NAME, OUTPUT, FILE, solve_day_8);
        assert_eq!(21, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(7, NAME, OUTPUT, FILE, solve_day_8);
        assert_eq!(8, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
