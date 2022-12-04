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

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use adventofcode2022_lib::utils::print_result;

const EXAMPLE_FILE: &str = "data/calories_example.dat";
const PUZZLE_FILE: &str = "data/calories_puzzle.dat";

fn main() {
    println!("Advent of Code 2022 - Day 1: Calorie Counting");
    println!();
    print_result(
        "part 1 [example]",
        "calories",
        solve_day_1_part_1(EXAMPLE_FILE),
    );
    print_result(
        "part 1 [puzzle]",
        "calories",
        solve_day_1_part_1(PUZZLE_FILE),
    );
    print_result(
        "part 2 [example]",
        "calories",
        solve_day_1_part_2(EXAMPLE_FILE),
    );
    print_result(
        "part 2 [puzzle]",
        "calories",
        solve_day_1_part_2(PUZZLE_FILE),
    );
}

fn solve_day_1_part_1(filename: &str) -> u32 {
    solve_day_1(filename, 1)
}

fn solve_day_1_part_2(filename: &str) -> u32 {
    solve_day_1(filename, 3)
}

fn solve_day_1(filename: &str, top: usize) -> u32 {
    // total calories in the current elf
    let mut current_elf_total_calories = 0;
    // total calories per elf
    let mut calories_per_elf: Vec<u32> = Vec::new();

    //open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // line empty, we handle previous elf totals
        if line.is_empty() {
            calories_per_elf.push(current_elf_total_calories);
            // next elf stars with 0 calories
            current_elf_total_calories = 0;
        } else {
            // add the calories to the current elf
            current_elf_total_calories += line.parse::<u32>().unwrap();
        }
    }
    // sort and get the sum of the n calories
    sum_top_n_calories(calories_per_elf, top)
}

// sum the top n calories
fn sum_top_n_calories(calories: Vec<u32>, top: usize) -> u32 {
    let mut copy = calories.clone();
    copy.sort();
    copy.iter().rev().take(top).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        let total = solve_day_1_part_1(EXAMPLE_FILE);
        assert_eq!(total, 24000);
    }

    #[test]
    fn test_part_2() {
        let total = solve_day_1_part_2(EXAMPLE_FILE);
        assert_eq!(total, 45000);
    }
}
