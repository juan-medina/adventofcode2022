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

mod monkey;

use crate::monkey::Monkey;
use adventofcode2022_lib::utils::{read_file, Example, RunType};

const NAME: &'static str = "Monkey in the Middle";
const OUTPUT: &'static str = "monkey business";
const FILE: &'static str = "monkey_notes";

fn main() {
    Example::new(11, NAME, OUTPUT, FILE, solve_day_11).run_all();
}

fn solve_day_11(filename: &str, run_type: RunType) -> usize {
    let mut count = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();

    let calm_down = match run_type {
        RunType::Part1 => true,
        RunType::Part2 => false,
    };

    let rounds = match run_type {
        RunType::Part1 => 20,
        RunType::Part2 => 10000,
    };

    let mut lines: String = String::from("");
    let mut base = 1;
    for line in read_file(filename) {
        count += 1;
        if count == 7 {
            let monkey = Monkey::new(&lines);
            base *= monkey.divisible;
            monkeys.push(monkey);
            count = 0;
            lines = String::from("");
        } else {
            lines += (line + "\n").as_str();
        }
    }

    for _ in 1..rounds + 1 {
        for monkey_num in 0..monkeys.len() {
            while monkeys[monkey_num].items.len() > 0 {
                let (new_item, to_monkey) = monkeys[monkey_num].inspect(calm_down, base);
                monkeys[to_monkey].items.push_back(new_item);
            }
        }
    }

    monkeys.sort_by(monkey::sort_monkeys_by_inspects_reverse);

    &monkeys[0].inspects * &monkeys[1].inspects
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(11, NAME, OUTPUT, FILE, solve_day_11);
        assert_eq!(
            10605,
            example.run_part(FileType::ExampleFile, RunType::Part1)
        );
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(11, NAME, OUTPUT, FILE, solve_day_11);
        assert_eq!(
            2713310158,
            example.run_part(FileType::ExampleFile, RunType::Part2)
        );
    }
}
