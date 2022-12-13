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
