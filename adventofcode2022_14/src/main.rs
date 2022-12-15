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

use adventofcode2022_lib::utils::{Example, FileType, RunType};

mod map;
mod parser;
mod points;

const NUM: &'static usize = &14;
const NAME: &'static str = "Regolith Reservoir";
const OUTPUT: &'static [&'static str] = &["units sand resting", "units until clog"];
const FILE: &'static str = "lines";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_14).run_all();
}

fn solve_day_14(filename: &str, run_type: RunType, _: FileType) -> usize {
    let points_sets = parser::parse(filename);
    let to_drop_point = match run_type {
        RunType::Part1 => false,
        RunType::Part2 => true,
    };
    let (sand_drop, mut map) = map::create(&points_sets, to_drop_point);

    let mut counter = 0;
    while map::drop_sand(sand_drop, &mut map, to_drop_point) {
        counter += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_14);
        assert_eq!(24, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_14);
        assert_eq!(93, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
