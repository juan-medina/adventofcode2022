use crate::item::Item;
use crate::item::Item::{Arr, Empty, Num};
use adventofcode2022_lib::utils::read_file;
use regex::Regex;

pub fn parse(filename: &str) -> Vec<Item> {
    let mut result = vec![];
    let re_parse = Regex::new(r"[\[\]]|\d+").unwrap();

    let mut stack = vec![];

    let mut current = Empty;

    for line in read_file(filename) {
        for capture in re_parse.captures_iter(&line) {
            match &capture[0] {
                "[" => {
                    let new_list = Arr(vec![]);
                    match current {
                        Empty => {
                            current = new_list;
                        }
                        Arr(_) => {
                            stack.push(current);
                            current = new_list;
                        }
                        _ => {}
                    }
                }
                "]" => match current {
                    Arr(_) => {
                        if stack.len() != 0 {
                            let mut previous = stack.pop().unwrap();
                            if let Arr(arr) = &mut previous {
                                arr.push(current);
                                current = previous;
                            }
                        } else {
                            result.push(current);
                            current = Empty;
                        }
                    }
                    _ => {}
                },
                num => match &mut current {
                    Arr(li) => {
                        li.push(Num(num.parse::<i32>().unwrap()));
                    }
                    _ => {}
                },
            }
        }
    }
    result
}
