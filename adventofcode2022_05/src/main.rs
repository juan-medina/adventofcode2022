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
    collections::LinkedList,
};

use regex::Regex;

use adventofcode2022_lib::utils::print_result;

const EXAMPLE_FILE: &str = "data/cargo_example.dat";
const PUZZLE_FILE: &str = "data/cargo_puzzle.dat";

// What crane model we use, 900 do one crate at a time
// 9001 do all at the same time
enum CraneModel {
    CrateMover9000,
    CrateMover9001,
}

fn main() {
    println!("Day 5: Supply Stacks");
    println!();
    print_result(
        "part 1 [example]",
        "top crates said",
        solve_day_5_part_1(EXAMPLE_FILE),
    );
    print_result(
        "part 1 [puzzle]",
        "top crates said",
        solve_day_5_part_1(PUZZLE_FILE),
    );
    print_result(
        "part 1 [example]",
        "top crates said",
        solve_day_5_part_2(EXAMPLE_FILE),
    );
    print_result(
        "part 1 [puzzle]",
        "top crates said",
        solve_day_5_part_2(PUZZLE_FILE),
    );
}

fn solve_day_5_part_1(filename: &str) -> String {
    solve_day_5(filename, CraneModel::CrateMover9000)
}

fn solve_day_5_part_2(filename: &str) -> String {
    solve_day_5(filename, CraneModel::CrateMover9001)
}

fn solve_day_5(filename: &str, crane_model: CraneModel) -> String {
    // lines that we are reading
    let mut lines: Vec<String> = Vec::new();

    // our cargo
    let mut cargo: Vec<LinkedList<char>> = vec![];

    //open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // when reaching an empty line we complete creates
        if line == "" {
            cargo = process_crates(lines.clone());
            lines.clear();
            continue;
        }

        // add new line to the current reading
        lines.push(line);
    }
    // we complete reading moves process then
    cargo = process_moves(cargo, lines, crane_model);

    process_result(cargo)
}

fn process_result(cargo: Vec<LinkedList<char>>) -> String {
    let mut result = String::from("");

    for crates in cargo {
        let cargo_value = crates.front().unwrap();
        result.push(cargo_value.clone());
    }

    String::from(result)
}

fn process_crates(lines: Vec<String>) -> Vec<LinkedList<char>> {
    // get the crates from the last line
    let crates_info = read_crates_line(lines[lines.len() - 1].clone());

    // num of crates
    let num_crates = crates_info.len();

    // here we will have our result, we initialize it
    let mut cargo_crates: Vec<LinkedList<char>> = Vec::new();
    for _num_crate in 0..num_crates {
        cargo_crates.push(LinkedList::new());
    }

    // get the rest of the line
    for num_line in 0..lines.len() - 1 {
        // read the crates
        let crates = read_crates_line(lines[num_line].clone());

        // go trough the crate and if not empty add it
        for crate_index in 0..crates.len() {
            let crate_value = crates[crate_index];
            if crate_value != ' ' {
                cargo_crates[crate_index].push_back(crate_value);
            }
        }
    }

    // return the crates
    cargo_crates
}

fn read_crates_line(line: String) -> Vec<char> {
    // our crate values
    let mut crate_values: Vec<char> = Vec::new();

    // collect characters from the line
    let characters: Vec<char> = line.chars().collect();

    // iterate through the crates
    for index in (0..line.len()).step_by(4) {
        let crate_value = characters[index + 1];
        crate_values.push(crate_value);
    }

    crate_values
}

fn process_moves(
    mut cargo: Vec<LinkedList<char>>,
    lines: Vec<String>,
    crane_model: CraneModel,
) -> Vec<LinkedList<char>> {
    // regex to capture move, from, to
    let regex = Regex::new(r"(?m)move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)").unwrap();

    // go trough each line
    for line in lines {
        // capture the groups
        let caps = regex.captures(&line).unwrap();

        // get amount, from, to
        let amount = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        match crane_model {
            CraneModel::CrateMover9000 => {
                // do the move from one cargo to another, pushing always to the top
                for _move in 0..amount {
                    let value = cargo[from - 1].pop_front().unwrap();
                    cargo[to - 1].push_front(value);
                }
            }
            CraneModel::CrateMover9001 => {
                // move the n top crates to a holder
                let mut holder: Vec<char> = Vec::new();
                for _move in 0..amount {
                    let value = cargo[from - 1].pop_front().unwrap();
                    holder.push(value);
                }
                // reverse the holder ABC > CBA
                holder.reverse();
                //push then to the top, since they are reverse the retain the original order
                for value in holder {
                    cargo[to - 1].push_front(value);
                }
            }
        }
    }
    return cargo;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        let top_crates_said = solve_day_5_part_1(EXAMPLE_FILE);
        assert_eq!(top_crates_said, "CMZ");
    }

    #[test]
    fn test_part_2() {
        let top_crates_said = solve_day_5_part_2(EXAMPLE_FILE);
        assert_eq!(top_crates_said, "MCD");
    }
}
