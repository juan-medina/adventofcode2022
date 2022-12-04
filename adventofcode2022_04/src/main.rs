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

mod ranges;

const EXAMPLE_FILE: &str = "data/pairs_example.dat";
const PUZZLE_FILE: &str = "data/pairs_puzzle.dat";

fn main() {
    println!("Day 4: Camp Cleanup");
    println!();
    print_result(
        "part 1 [example]",
        "couples fully contained",
        solve_day_4_part_1(EXAMPLE_FILE),
    );
    print_result(
        "part 1 [puzzle]",
        "couples fully contained",
        solve_day_4_part_1(PUZZLE_FILE),
    );
    print_result(
        "part 2 [example]",
        "couples colliding",
        solve_day_4_part_2(EXAMPLE_FILE),
    );
    print_result(
        "part 2 [puzzle]",
        "couples colliding",
        solve_day_4_part_2(PUZZLE_FILE),
    );
}

// what kind of check we are doing
enum CheckType {
    FullyContains,
    Overlap,
}

fn solve_day_4_part_1(filename: &str) -> u32 {
    solve_day_4(filename, CheckType::FullyContains)
}

fn solve_day_4_part_2(filename: &str) -> u32 {
    solve_day_4(filename, CheckType::Overlap)
}

fn solve_day_4(filename: &str, check_type: CheckType) -> u32 {
    // total couples in this check
    let mut total_couples = 0;

    //open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // get each elf sections
        let (first_elf, second_elf) = ranges::get_sections(line);

        // get if the sections are contained or overlapped
        let should_count = match check_type {
            CheckType::FullyContains => ranges::fully_contained(first_elf, second_elf),
            CheckType::Overlap => ranges::overlap(first_elf, second_elf),
        };

        // accumulate if should count
        if should_count {
            total_couples += 1;
        }
    }
    total_couples
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        let total = solve_day_4_part_1(EXAMPLE_FILE);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_part_2() {
        let total = solve_day_4_part_2(EXAMPLE_FILE);
        assert_eq!(total, 4);
    }
}
