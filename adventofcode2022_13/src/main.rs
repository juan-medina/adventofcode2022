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

mod item;
mod parser;

const NUM: &'static usize = &13;
const NAME: &'static str = "Distress Signal";
const OUTPUT: &'static [&'static str] = &["sum of indices", "decoder key"];
const FILE: &'static str = "packets";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_13).run_all();
}

fn solve_day_13(filename: &str, run_type: RunType, _: FileType) -> usize {
    let mut result = parser::parse(filename);

    let mut total = 0usize;
    match run_type {
        RunType::Part1 => {
            for (i, items) in result.chunks(2).enumerate() {
                let left = &items[0];
                let right = &items[1];
                if left <= right {
                    total += i + 1;
                }
            }
        }
        RunType::Part2 => {
            let divider_1 = item::create_divider(2);
            let divider_2 = item::create_divider(6);
            result.push(divider_1.clone());
            result.push(divider_2.clone());

            result.sort();

            total = 1;
            for (i, item) in result.iter().enumerate() {
                if item.eq(&divider_1.clone()) || item.eq(&divider_2.clone()) {
                    total *= i + 1;
                }
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
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_13);
        assert_eq!(13, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_13);
        assert_eq!(140, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
