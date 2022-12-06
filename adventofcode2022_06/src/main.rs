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

use adventofcode2022_lib::utils::{has_duplicates, print_result, read_whole_file};

const EXAMPLE_FILE: &str = "data/signal_example.dat";
const PUZZLE_FILE: &str = "data/signal_puzzle.dat";

enum MarkerType {
    Packet = 4,
    Message = 14,
}

fn main() {
    println!("Day 6: Tuning Trouble");
    println!();
    print_result(
        "part 1 [example]",
        "first marker after character",
        solve_day_6_part_1(EXAMPLE_FILE),
    );
    print_result(
        "part 1 [puzzle]",
        "first marker after character",
        solve_day_6_part_1(PUZZLE_FILE),
    );
    print_result(
        "part 2 [example]",
        "first message after character",
        solve_day_6_part_2(EXAMPLE_FILE),
    );
    print_result(
        "part 2 [puzzle]",
        "first message after character",
        solve_day_6_part_2(PUZZLE_FILE),
    );
}

fn solve_day_6_part_1(filename: &str) -> usize {
    solve_day_6(filename, MarkerType::Packet)
}

fn solve_day_6_part_2(filename: &str) -> usize {
    solve_day_6(filename, MarkerType::Message)
}

fn solve_day_6(filename: &str, marker_type: MarkerType) -> usize {
    //open the file and read it as whole
    let contents = read_whole_file(filename);

    // find the marker
    find_marker(contents, marker_type as usize)
}

// find a marker of a given len
fn find_marker(signal: String, marker_len: usize) -> usize {
    let chars: Vec<char> = signal.chars().collect();

    // iterate trough the signal and check if we have a marker
    for index in 0..(chars.len() - marker_len) {
        let pos = index + marker_len;
        let marker = &chars[index..pos];
        if !has_duplicates(marker) {
            return pos;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_day_6_part_1(EXAMPLE_FILE), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_day_6_part_2(EXAMPLE_FILE), 19);
    }
}
