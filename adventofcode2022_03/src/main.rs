use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {

    // sum of all priorities
    let mut sum_all_priorities = 0;

    //open the file
    let filename = "data/rucksacks.dat";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // read the file line by line
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        // splits rucksacks in two compartment
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

        // find the repeat type
        let repeated_type = repeated_type(first_compartment, second_compartment);

        // get the unicode character value
        let mut priority = repeated_type as u32;

        // calculate priority
        if repeated_type.is_lowercase() {
            //for lowercase is 1..26 (a..z)
            priority = priority - 96;
        }else{
            // for uppercase is 27..52 (A..Z)
            priority = priority - 64 + 26;
        }

        // accumulate priority
        sum_all_priorities += priority;

        // print stats
        println!("rucksacks: {line}, first compartment {first_compartment} - second_compartment {second_compartment}, repeated type {repeated_type} - priority {priority}");
    }
    println!("Summ of all priorities: {sum_all_priorities}");
}

fn repeated_type(first_compartment: &str, second_compartment: &str) -> char {
    let mut found = ' ';
    for item_type in first_compartment.chars() {
        if second_compartment.find(item_type) != None {
            found = item_type;
            break;
        }
    }
    return found
}

