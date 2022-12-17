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

mod cave;
mod parser;
mod valve;

use adventofcode2022_lib::utils::{Example, FileType, RunType};

const NUM: &'static usize = &16;
const NAME: &'static str = "Proboscidea Volcanium";
const OUTPUT: &'static [&'static str] = &["most pressure released", "most pressure released"];
const FILE: &'static str = "valves";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_16).run_all();
}

fn solve_day_16(filename: &str, run_type: RunType, _file_type: FileType) -> u32 {
    let mut cave = cave::new(parser::new(filename).parse());

    match run_type {
        RunType::Part1 => cave.max_pressure(false),
        RunType::Part2 => cave.max_pressure(true),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_16);
        assert_eq!(
            1651,
            example.run_part(FileType::ExampleFile, RunType::Part1)
        );
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_16);
        assert_eq!(
            1707,
            example.run_part(FileType::ExampleFile, RunType::Part2)
        );
    }
}
