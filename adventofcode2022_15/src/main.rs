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

mod map;
mod parser;
mod point;
mod ranges;
mod sensor;

use adventofcode2022_lib::utils::RunType::{Part1, Part2};
use adventofcode2022_lib::utils::{Example, FileType, RunType};

const NUM: &'static usize = &15;
const NAME: &'static str = "Beacon Exclusion Zone";
const OUTPUT: &'static [&'static str] = &["positions with not beacons on", "hidden beacon"];
const FILE: &'static str = "sensors";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_15).run_all();
}

fn solve_day_15(filename: &str, run_type: RunType, file_type: FileType) -> String {
    let parser = parser::new(filename);
    let sensors = parser.parse();
    let map = map::new(sensors);

    return match run_type {
        Part1 => {
            let row = match file_type {
                FileType::ExampleFile => 10,
                FileType::PuzzleFile => 2000000,
            };

            let positions = map.positions_with_no_beacons(row);

            format!("row {} = {}", row, positions)
        }
        Part2 => {
            let max = match file_type {
                FileType::ExampleFile => 20,
                FileType::PuzzleFile => 4000000,
            };
            let freq = map.get_frequency_hidden_beacon(max);

            format!("freq {}", freq)
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_15);
        assert_eq!(
            "row 10 = 26",
            example.run_part(FileType::ExampleFile, RunType::Part1)
        );
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_15);
        assert_eq!(
            "freq 56000011",
            example.run_part(FileType::ExampleFile, RunType::Part2)
        );
    }
}
