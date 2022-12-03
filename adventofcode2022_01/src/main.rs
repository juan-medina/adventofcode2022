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
