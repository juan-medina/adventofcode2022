use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // current reindeer number
    let mut current_reindeer_number = 1;
    // total calories in the current reindeer
    let mut current_reindeer_total_calories = 0;
    // max calories for all the reindeers
    let mut max_calories_in_all_reindeers = 0;
    // which reindeer has the most calories
    let mut reindeer_with_most_calories = 0;

    //open the file
    let filename = "data/calories.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // line empty, we handle previous reindeer totals
        if line.is_empty() {
            // print previous reindeer stats
            println!(
                "reindeer number {} has {} total calories",
                current_reindeer_number, current_reindeer_total_calories
            );
            // if we have more calories that the current max, we set this as the reindeer with most calories
            if current_reindeer_total_calories > max_calories_in_all_reindeers {
                reindeer_with_most_calories = current_reindeer_number;
                max_calories_in_all_reindeers = current_reindeer_total_calories;
            }
            // next reindeer stars with 0 calories
            current_reindeer_number += 1;
            current_reindeer_total_calories = 0;
        } else {
            // add the calories to the current reindeer
            let calories: i32 = line.parse().unwrap();
            current_reindeer_total_calories += calories;
        }
    }
    println!(
        "The reindeer with most calories is the number {} with {} calories",
        reindeer_with_most_calories, max_calories_in_all_reindeers
    );
}
