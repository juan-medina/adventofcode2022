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
    // current elf number
    let mut current_elf_number = 1;
    // total calories in the current elf
    let mut current_elf_total_calories = 0;
    // total calories per elf
    let mut calories_per_elf: Vec<i32> = Vec::new();

    //open the file
    let filename = "data/calories.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // line empty, we handle previous elf totals
        if line.is_empty() {
            // print previous elf stats
            println!(
                "elf number {} has {} total calories",
                current_elf_number, current_elf_total_calories
            );
            calories_per_elf.push(current_elf_total_calories);
            // next elf stars with 0 calories
            current_elf_number += 1;
            current_elf_total_calories = 0;
        } else {
            // add the calories to the current elf
            current_elf_total_calories += line.parse::<i32>().unwrap();
        }
    }
    // sort and get the sum of the 3 higher calories
    println!("sum of top 3 calories {}", sum_top_n_calories(calories_per_elf, 3));
}

// sum the top n calories
fn sum_top_n_calories(calories: Vec<i32>, top: usize) -> i32 {
    let mut copy = calories.clone();
    copy.sort();
    copy.iter().rev().take(top).sum()
}
