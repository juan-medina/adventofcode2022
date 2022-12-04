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

use std::cmp::{max, min};
use std::ops::Range;

// check if two ranges are fully contained, first could contain second or reverse
pub fn fully_contained(first: Range<u32>, second: Range<u32>) -> bool {
    range_contains(first.clone(), second.clone()) || range_contains(second, first)
}

// check if two ranges overlap
pub fn overlap(first: Range<u32>, second: Range<u32>) -> bool {
    max(first.start, second.start) <= min(first.end, second.end)
}

// check if a range contains another range
fn range_contains(first: Range<u32>, second: Range<u32>) -> bool {
    first.start <= second.start && first.end >= second.end
}

// create a range with a string, ex 8-12
fn get_range(token: &str) -> Range<u32> {
    let (begin_str, end_str) = token.split_once("-").unwrap();

    let begin = begin_str.parse::<u32>().unwrap();
    let end = end_str.parse::<u32>().unwrap();
    Range { start: begin, end }
}

// get the two range of sections from an string, ex: 8-12,20-30
pub fn get_sections(line: String) -> (Range<u32>, Range<u32>) {
    let (first_elf, second_elf) = line.split_once(",").unwrap();
    (get_range(first_elf), get_range(second_elf))
}
