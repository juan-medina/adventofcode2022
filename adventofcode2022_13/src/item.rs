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
