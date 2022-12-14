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

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Empty,
    Num(i32),
    Arr(Vec<Item>),
}

fn create_item(item: Item) -> Item {
    Item::Arr(vec![item])
}

pub fn create_divider(num: i32) -> Item {
    create_item(create_item(Item::Num(num)))
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Item::*;
        match (self, other) {
            (Empty, Empty) => Some(Equal),
            (Empty, _) => Some(Less),
            (_, Empty) => Some(Greater),
            (Num(n1), Num(n2)) => n1.partial_cmp(n2),
            (Num(n), Arr(arr)) => vec![Num(*n)].partial_cmp(arr),
            (Arr(arr), Num(n)) => arr.partial_cmp(&vec![Num(*n)]),
            (Arr(arr1), Arr(arr2)) => arr1.partial_cmp(arr2),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
