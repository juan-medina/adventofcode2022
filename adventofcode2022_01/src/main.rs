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

use adventofcode2022_lib::utils;
use adventofcode2022_lib::utils::{Example, FileType, RunType};

const NUM: &'static usize = &1;
const NAME: &'static str = "Calorie Counting";
const OUTPUT: &'static [&'static str] = &["most calories", "total calories"];
const FILE: &'static str = "calories";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_1).run_all();
}

fn solve_day_1(filename: &str, run_type: RunType, _: FileType) -> u32 {
    // change the top values depending of Part 1 or 2
    let top: usize = match run_type {
        RunType::Part1 => 1,
        RunType::Part2 => 3,
    };

    // total calories in the current elf
    let mut current_elf_total_calories = 0;
    // total calories per elf
    let mut calories_per_elf: Vec<u32> = Vec::new();

    // read the file line by line
    for line in utils::read_file(filename) {
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
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_1);
        assert_eq!(
            24000,
            example.run_part(FileType::ExampleFile, RunType::Part1)
        );
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_1);
        assert_eq!(
            45000,
            example.run_part(FileType::ExampleFile, RunType::Part2)
        );
    }
}
