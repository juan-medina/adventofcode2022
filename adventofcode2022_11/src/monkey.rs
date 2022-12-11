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

const MONKEY_FORMAT: &str = r"(?m)^Monkey (\d+):
 {2}Starting items: ([0-9, ]+)
 {2}Operation: new = (.+) (.) (.+)
 {2}Test: divisible by (.+)
 {4}If true: throw to monkey (.+)
 {4}If false: throw to monkey (.+)
";

impl Monkey {
    pub fn new(lines: &String) -> Self {
        let captures = Regex::new(MONKEY_FORMAT).unwrap().captures(lines).unwrap();

        // id
        let id = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();

        // items
        let mut items: LinkedList<u64> = LinkedList::new();
        let items_str = captures.get(2).unwrap().as_str().replace(" ", "");
        for item in items_str.split(",") {
            let value = item.parse::<u64>().unwrap();
            items.push_back(value);
        }

        // operation
        let arg1 = String::from(captures.get(3).unwrap().as_str());
        let operation = String::from(captures.get(4).unwrap().as_str());
        let arg2 = String::from(captures.get(5).unwrap().as_str());

        // divisible
        let divisible = captures.get(6).unwrap().as_str().parse::<u64>().unwrap();

        // if true
        let if_true = captures.get(7).unwrap().as_str().parse::<usize>().unwrap();

        // if false
        let if_false = captures.get(8).unwrap().as_str().parse::<usize>().unwrap();

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

    pub fn inspect(&mut self, calm_down: bool, base: u64) -> (u64, usize) {
        let item = self.items.pop_front().unwrap();
        let arg1_value = value(&self.arg1, item);
        let arg2_value = value(&self.arg2, item);

        let mut worrying = if self.operation == "+" {
            arg1_value + arg2_value
        } else {
            arg1_value * arg2_value
        };

        worrying = if calm_down {
            worrying / 3
        } else {
            worrying % base
        };

        let throw = if (worrying % self.divisible) == 0 {
            self.if_true
        } else {
            self.if_false
        };

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
