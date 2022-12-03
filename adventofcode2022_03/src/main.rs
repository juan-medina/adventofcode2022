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
};

fn main() {
    // sum of all priorities
    let mut sum_all_priorities = 0;

    // the group of rucksacks
    let mut group_of_rucksacks: Vec<String> = Vec::new();

    // group of 3 rucksacks
    const RUCKSACKS_PER_GROUP: usize = 3;

    //open the file
    let filename = "data/rucksacks.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // print stats
        println!("rucksacks: {line}");

        // add the rucksack to the group
        group_of_rucksacks.push(line);

        // when we have all rucksacks in the group we complete the group
        if group_of_rucksacks.len() == RUCKSACKS_PER_GROUP {
            // find the repeat type
            let repeated_type = repeated_type(group_of_rucksacks.clone());

            // get the priority
            let priority = get_item_priority(repeated_type);

            // accumulate priority
            sum_all_priorities += priority;

            // clear this group
            group_of_rucksacks.clear();

            // print stats
            println!("end of group, repeated type {repeated_type} - priority {priority}");
        }
    }
    println!("Summ of all priorities: {sum_all_priorities}");
}

fn repeated_type(group_of_rucksacks: Vec<String>) -> char {
    // the repeated type
    let mut repeated_type = ' ';

    // get first rucksack
    let first_rucksack = group_of_rucksacks[0].clone();

    // must be in this amount of sacks
    let numbers_of_sacks_to_find = group_of_rucksacks.len();

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
