use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // sum of all priorities
    let mut sum_all_priorities = 0;

    // the group of 3 rucksacks
    let mut group_of_rucksacks: Vec<String> = Vec::new();

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

        // when we have 3 rucksacks in the group we complete the group
        if group_of_rucksacks.len() == 3 {
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
    // get each rucksack
    let first_rucksack = group_of_rucksacks[0].clone();
    let second_rucksack = group_of_rucksacks[1].clone();
    let third_rucksack = group_of_rucksacks[2].clone();

    // the repeated type
    let mut repeated_type = ' ';

    // iterate the first rucksack and check if found in the others
    for item_type in first_rucksack.chars() {
        if second_rucksack.find(item_type) != None {
            if third_rucksack.find(item_type) != None {
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

