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

use adventofcode2022_lib::utils::{read_file, Example, FileType, RunType};
use std::collections::{HashMap, LinkedList};

use regex::Regex;

const NUM: &'static usize = &7;
const NAME: &'static str = "No Space Left On Device";
const OUTPUT: &'static [&'static str] = &["sum of the total sizes", "size of directory to free"];
const FILE: &'static str = "commands";

fn main() {
    Example::new(NUM, NAME, OUTPUT, FILE, solve_day_7).run_all();
}

fn solve_day_7(filename: &str, _run_type: RunType, _: FileType) -> u32 {
    // represent a cd command
    let re_cd_cmd = Regex::new(r"(?m)^\$ cd ([/.a-z]+)").unwrap();
    // represent a file
    let re_file = Regex::new(r"(?m)^(\d+) ([a-z]+.?[a-z]*)").unwrap();

    // our current path
    let mut current_path: LinkedList<String> = LinkedList::new();

    // each directory size
    let mut dir_size: HashMap<String, u32> = HashMap::new();

    //open the file and read it as whole
    for line in read_file(filename) {
        // if we have a cd command or a file
        if re_cd_cmd.is_match(&line) {
            // get the directory name
            let caps = re_cd_cmd.captures(&line).unwrap();
            let dir_name = caps.get(1).unwrap().as_str();

            // if is back remove directory from the path
            //  if not add it
            if dir_name != ".." {
                current_path.push_back(dir_name.to_string());
            } else {
                current_path.pop_back();
            }
        } else if re_file.is_match(&line) {
            // get the size and name
            let caps = re_file.captures(&line).unwrap();
            let size = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let _name = caps.get(2).unwrap().as_str();

            // get the current path
            let mut path_walk = current_path.clone();
            // walk backwards int he path
            let steps = path_walk.len();
            // add the file size to all path that contains it
            for _i in 0..steps {
                let full_path = get_full_path(path_walk.clone());
                *dir_size.entry(full_path).or_insert(0) += size;
                path_walk.pop_back();
            }
        }
    }

    match _run_type {
        RunType::Part1 => {
            // our total sum
            let mut total_sum = 0;
            const THRESHOLD: u32 = 100000;

            // got trough all directories
            for item in dir_size.iter() {
                let value = item.1;
                // if its within the threshold add to the sum
                if *value <= THRESHOLD {
                    total_sum += value;
                }
            }
            total_sum
        }
        RunType::Part2 => {
            const TOTAL_SPACE: u32 = 70000000;
            const REQUIRE_UNUSED_SPACE: u32 = 30000000;

            // calculate how much space we need to free
            let current_space = dir_size["/"];
            let free = TOTAL_SPACE - current_space;
            let to_free = REQUIRE_UNUSED_SPACE - free;

            // get the values from the map
            let mut values: Vec<u32> = dir_size.values().cloned().collect();

            // sort then and get the smaller that can free the space required
            values.sort();
            for value in values {
                if value >= to_free as u32 {
                    return value;
                }
            }
            0
        }
    }
}

// get a full path in a string
fn get_full_path(path: LinkedList<String>) -> String {
    let mut result: String = String::from("/");
    for part in path.iter().skip(1) {
        result += part;
        result += "/";
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_7);
        assert_eq!(
            95437,
            example.run_part(FileType::ExampleFile, RunType::Part1)
        );
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(NUM, NAME, OUTPUT, FILE, solve_day_7);
        assert_eq!(
            24933642,
            example.run_part(FileType::ExampleFile, RunType::Part2)
        );
    }
}
