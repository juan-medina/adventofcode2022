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

use regex::Regex;
use std::cmp::Ordering;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: LinkedList<u64>,
    pub operation: String,
    pub arg1: String,
    pub arg2: String,
    pub divisible: u64,
    pub if_true: usize,
    pub if_false: usize,
    pub inspects: usize,
}

impl Monkey {
    pub fn new(lines: &Vec<String>) -> Self {
        // id
        let re_number = Regex::new(r"(?m)^Monkey (\d+):").unwrap();
        let caps = re_number.captures(&lines[0]).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

        // items
        let mut items: LinkedList<u64> = LinkedList::new();
        let re_items = Regex::new(r"(?m)^  Starting items: ([0-9, ]+)").unwrap();
        let caps = re_items.captures(&lines[1]).unwrap();
        let items_str = caps.get(1).unwrap().as_str().replace(" ", "");
        for item in items_str.split(",") {
            let value = item.parse::<u64>().unwrap();
            items.push_back(value);
        }

        // operation
        let re_operation = Regex::new(r"(?m)^  Operation: new = (.+) (.) (.+)").unwrap();
        let caps = re_operation.captures(&lines[2]).unwrap();
        let arg1 = String::from(caps.get(1).unwrap().as_str());
        let operation = String::from(caps.get(2).unwrap().as_str());
        let arg2 = String::from(caps.get(3).unwrap().as_str());

        // divisible
        let re_divisible = Regex::new(r"(?m)^  Test: divisible by (.+)").unwrap();
        let caps = re_divisible.captures(&lines[3]).unwrap();
        let divisible = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();

        // if true
        let re_re_if_true = Regex::new(r"(?m)^    If true: throw to monkey (.+)").unwrap();
        let caps = re_re_if_true.captures(&lines[4]).unwrap();
        let if_true = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

        // if false
        let re_re_if_false = Regex::new(r"(?m)^    If false: throw to monkey (.+)").unwrap();
        let caps = re_re_if_false.captures(&lines[5]).unwrap();
        let if_false = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

        // inspects
        let inspects = 0;

        Self {
            id,
            items,
            operation,
            arg1,
            arg2,
            divisible,
            if_true,
            if_false,
            inspects,
        }
    }

    pub fn inspect(&mut self, old: u64, calm_down: bool, base: u64) -> (u64, usize) {
        let arg1_value = value(&self.arg1, old);
        let arg2_value = value(&self.arg2, old);

        let mut worrying: u64;

        if self.operation == "+" {
            worrying = arg1_value + arg2_value;
        } else {
            worrying = arg1_value * arg2_value;
        }

        if calm_down {
            worrying /= 3;
        } else {
            worrying = worrying % base;
        }

        let throw: usize;

        if (worrying % self.divisible) == 0 {
            throw = self.if_true;
        } else {
            throw = self.if_false;
        }

        self.inspects += 1;

        return (worrying, throw);
    }
}

fn value(arg: &String, old: u64) -> u64 {
    if arg.chars().next().unwrap().is_digit(10) {
        arg.parse().unwrap()
    } else {
        old
    }
}

pub fn sort_monkeys_by_inspects_reverse(left: &Monkey, right: &Monkey) -> Ordering {
    if left.inspects < right.inspects {
        Ordering::Greater
    } else if left.inspects > right.inspects {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn modulus_test() {
        let divided_by_3 = 3;
        let divided_by_4 = 4;
        let divided_by_7 = 7;
        let divided_by_9 = 9;

        let mut num_84000 = 84000;
        let mut num_12000 = 12000;
        let mut num_28000 = 28000;

        assert_eq!(
            true,
            num_84000 % divided_by_3 == 0,
            "84000 is divisible by 3"
        );
        assert_eq!(
            true,
            num_84000 % divided_by_4 == 0,
            "84000 is divisible by 4"
        );
        assert_eq!(
            true,
            num_84000 % divided_by_7 == 0,
            "84000 is divisible by 7"
        );
        assert_eq!(
            false,
            num_84000 % divided_by_9 == 0,
            "84000 is not divisible by 9"
        );

        assert_eq!(
            true,
            num_12000 % divided_by_3 == 0,
            "12000 is divisible by 3"
        );
        assert_eq!(
            true,
            num_12000 % divided_by_4 == 0,
            "12000 is divisible by 4"
        );
        assert_eq!(
            false,
            num_12000 % divided_by_7 == 0,
            "12000 is not divisible by 7"
        );
        assert_eq!(
            false,
            num_12000 % divided_by_9 == 0,
            "12000 is not divisible by 9"
        );

        assert_eq!(
            false,
            num_28000 % divided_by_3 == 0,
            "28000 is not divisible by 3"
        );
        assert_eq!(
            true,
            num_28000 % divided_by_4 == 0,
            "28000 is divisible by 4"
        );
        assert_eq!(
            true,
            num_28000 % divided_by_7 == 0,
            "28000 is divisible by 7"
        );
        assert_eq!(
            false,
            num_28000 % divided_by_9 == 0,
            "28000 is not divisible by 9"
        );

        // base of all divisible
        let base = divided_by_3 * divided_by_4 * divided_by_7 * divided_by_9;

        // reduce numbers to the base if they where divisible the still be
        num_84000 = num_84000 % base; // 84
        num_12000 = num_12000 % base; // 660
        num_28000 = num_28000 % base; // 28

        assert_eq!(
            true,
            num_84000 % divided_by_3 == 0,
            "reduced 84000 is divisible by 3"
        );
        assert_eq!(
            true,
            num_84000 % divided_by_4 == 0,
            "reduced 84000 is divisible by 4"
        );
        assert_eq!(
            true,
            num_84000 % divided_by_7 == 0,
            "reduced 84000 is divisible by 7"
        );
        assert_eq!(
            false,
            num_84000 % divided_by_9 == 0,
            "reduced 84000 is not divisible by 9"
        );

        assert_eq!(
            true,
            num_12000 % divided_by_3 == 0,
            "reduced 12000 is divisible by 3"
        );
        assert_eq!(
            true,
            num_12000 % divided_by_4 == 0,
            "reduced 12000 is divisible by 4"
        );
        assert_eq!(
            false,
            num_12000 % divided_by_7 == 0,
            "reduced 12000 is not divisible by 7"
        );
        assert_eq!(
            false,
            num_12000 % divided_by_9 == 0,
            "reduced 12000 is not divisible by 9"
        );

        assert_eq!(
            false,
            num_28000 % divided_by_3 == 0,
            "reduced 28000 is not divisible by 3"
        );
        assert_eq!(
            true,
            num_28000 % divided_by_4 == 0,
            "reduced 28000 is divisible by 4"
        );
        assert_eq!(
            true,
            num_28000 % divided_by_7 == 0,
            "reduced 28000 is divisible by 7"
        );
        assert_eq!(
            false,
            num_28000 % divided_by_9 == 0,
            "reduced 28000 is not divisible by 9"
        );
    }
}
