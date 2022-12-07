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

const NAME: &'static str = "Rucksack Reorganization";
const OUTPUT: &'static str = "priorities";
const FILE: &'static str = "rucksacks";

fn main() {
    Example::new(3, NAME, OUTPUT, FILE, solve_day_3).run_all();
}

fn solve_day_3(filename: &str, run_type: RunType) -> u32 {
    // how many rucksacks we have for group
    let rucksacks_per_group: usize = match run_type {
        RunType::Part1 => 1,
        RunType::Part2 => 3,
    };

    // sum of all priorities
    let mut sum_all_priorities = 0;

    // the group of rucksacks
    let mut group_of_rucksacks: Vec<String> = Vec::new();

    // read the file line by line
    for line in utils::read_file(filename) {
        // add the rucksack to the group
        group_of_rucksacks.push(line);

        // when we have all rucksacks in the group we complete the group
        if group_of_rucksacks.len() == rucksacks_per_group {
            // find the repeat type
            let repeated_type = repeated_type_in_groups(group_of_rucksacks.clone());

            // get the priority
            let priority = get_item_priority(repeated_type);

            // accumulate priority
            sum_all_priorities += priority;

            // clear this group
            group_of_rucksacks.clear();
        }
    }
    sum_all_priorities
}

fn repeated_type_in_compartments(first_compartment: &str, second_compartment: &str) -> char {
    let mut found = ' ';
    for item_type in first_compartment.chars() {
        if second_compartment.find(item_type) != None {
            found = item_type;
            break;
        }
    }
    return found;
}

fn repeated_type_in_groups(group_of_rucksacks: Vec<String>) -> char {
    // the repeated type
    let mut repeated_type = ' ';

    // must be in this amount of sacks
    let numbers_of_sacks_to_find = group_of_rucksacks.len();

    // get first rucksack
    let first_rucksack = group_of_rucksacks[0].clone();

    // if we are doing groups of 1 rucksack we check compartments, if not groups
    if numbers_of_sacks_to_find == 1 {
        // splits rucksacks in two compartment
        let (first, second) = first_rucksack.split_at(first_rucksack.len() / 2);
        // find between those
        repeated_type = repeated_type_in_compartments(first, second);
    } else {
        // iterate the first rucksack and check if found in the others
        for item_type in first_rucksack.chars() {
            // is found in the first sack
            let mut found_in = 1;
            // check the remaining sacks
            for i in 1..numbers_of_sacks_to_find {
                let sack = group_of_rucksacks[i].clone();
                // if is found we accumulate one found, if not we continue with the next item
                if sack.find(item_type) != None {
                    found_in += 1;
                } else {
                    break;
                }
            }
            // if its found in all sacks
            if found_in == numbers_of_sacks_to_find {
                repeated_type = item_type;
                break;
            }
        }
    }

    return repeated_type;
}

fn get_item_priority(item_type: char) -> u32 {
    // get the unicode character value
    let unicode_value = item_type as u32;

    // calculate priority
    if item_type.is_lowercase() {
        //for lowercase is 1..26 (a..z), ex: (a,b,c)=(1,2,3)
        const A_LOWER_UNICODE: u32 = 'a' as u32;
        // ex: 'c' unicode:99 - 'a' unicode:97 = 2, + 1 = 3
        unicode_value - A_LOWER_UNICODE + 1
    } else {
        // for uppercase is 27..52 (A..Z), ex: (A,B,C)=(27,28,29)
        const A_UPPER_UNICODE: u32 = 'A' as u32;
        const TOTAL_LETTERS: u32 = 26;
        // ex: 'C' unicode:67 - 'A' unicode:65 = 2, + 1 = 3, + 26 = 29
        unicode_value - A_UPPER_UNICODE + 1 + TOTAL_LETTERS
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(3, NAME, OUTPUT, FILE, solve_day_3);
        assert_eq!(157, example.run_part(FileType::ExampleFile, RunType::Part1));
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(3, NAME, OUTPUT, FILE, solve_day_3);
        assert_eq!(70, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
