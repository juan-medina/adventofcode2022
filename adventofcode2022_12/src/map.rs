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

use crate::node::Node;
use crate::step::NavigationStep;
use adventofcode2022_lib::utils::read_file;

static START_SYMBOL: char = 'S';
static END_SYMBOL: char = 'E';

pub static START_HEIGHT: usize = 'a' as usize - 1;
static END_HEIGHT: usize = 'z' as usize + 1;

pub static LOWEST_HEIGHT: usize = 'a' as usize;

pub fn get_map(filename: &str) -> (NavigationStep, Vec<Vec<usize>>) {
    let mut end_node = Node::zero();

    let mut x = 0usize;
    let mut y = 0usize;
    let mut map: Vec<Vec<usize>> = Vec::new();
    for line in read_file(filename) {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            let value = if c == START_SYMBOL {
                START_HEIGHT
            } else if c == END_SYMBOL {
                end_node = Node::new(x, y);
                END_HEIGHT
            } else {
                c as usize
            };
            row.push(value);
            x += 1;
        }
        map.push(row);
        y += 1;
        x = 0;
    }
    (end_node.step(0), map)
}
