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
use adventofcode2022_lib::utils::{Example, RunType};

mod ranges;

const NAME: &'static str = "Camp Cleanup";
const OUTPUT: &'static str = "couples";
const FILE: &'static str = "pairs";

fn main() {
    Example::new(3, NAME, OUTPUT, FILE, solve_day_4).run_all();
}

// what kind of check we are doing
enum CheckType {
    FullyContains,
    Overlap,
}

fn solve_day_4(filename: &str, run_type: RunType) -> u32 {
    // what type of check we do
    let check_type: CheckType = match run_type {
        RunType::Part1 => CheckType::FullyContains,
        RunType::Part2 => CheckType::Overlap,
    };

    // total couples in this check
    let mut total_couples = 0;

    // read the file line by line
    for line in utils::read_file(filename) {
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
    use adventofcode2022_lib::utils::FileType;
    use crate::*;

    #[test]
    fn test_part_1() {
        let example = Example::new(3, NAME, OUTPUT, FILE, solve_day_4);
        assert_eq!(2, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(3, NAME, OUTPUT, FILE, solve_day_4);
        assert_eq!(4, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
