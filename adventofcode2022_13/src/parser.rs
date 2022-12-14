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
