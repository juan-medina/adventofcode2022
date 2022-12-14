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

use crate::map::MapValue::{Empty, Rock, Sand};
use crate::points::{normalize, Point, Points};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapValue {
    Empty,
    Rock,
    Sand,
}

pub(crate) const DROP_X: i32 = 500i32;
pub(crate) const DROP_Y: i32 = 0i32;

impl fmt::Display for MapValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Empty => write!(f, "."),
            Rock => write!(f, "#"),
            Sand => write!(f, "o"),
        }
    }
}

pub fn create(points_sets: &Vec<Points>, extended: bool) -> (Point, Vec<Vec<MapValue>>) {
    let mut copy = points_sets.clone();
    let (reduced_by, size) = normalize(&mut copy, extended);

    let mut map: Vec<Vec<MapValue>> = vec![vec![Empty; size.x as usize]; size.y as usize];

    for i in 0..copy.len() {
        let mut iter = copy[i].iter();
        let mut from: &Point = iter.next().unwrap();
        for to in iter {
            draw_line(&mut map, from, to);
            from = &to;
        }
    }

    if extended {
        draw_h_line(&mut map, &Point::new(0, size.y - 1), size.x - 1);
    }

    (Point::new(DROP_X - reduced_by, DROP_Y), map)
}

pub fn draw_h_line(map: &mut Vec<Vec<MapValue>>, from: &Point, amount: i32) {
    for x in from.x..from.x + amount + 1 {
        map[from.y as usize][x as usize] = Rock;
    }
}

pub fn draw_v_line(map: &mut Vec<Vec<MapValue>>, from: &Point, amount: i32) {
    for y in from.y..from.y + amount + 1 {
        map[y as usize][from.x as usize] = Rock;
    }
}

pub fn draw_line(map: &mut Vec<Vec<MapValue>>, from: &Point, to: &Point) {
    //map[from.y as usize][from.x as usize] = Rock;
    //map[to.y as usize][to.x as usize] = Rock;
    let diff_y = from.y as i32 - to.y as i32;
    let diff_x = from.x as i32 - to.x as i32;
    let mut origin = Point::new(from.x, from.y);
    if diff_y == 0 {
        let distance = diff_x.abs() as i32;
        if from.x > to.x {
            origin.x = to.x;
        }
        draw_h_line(map, &origin, distance);
    } else {
        let distance = diff_y.abs() as i32;
        if from.y > to.y {
            origin.y = to.y;
        }
        draw_v_line(map, &origin, distance);
    }
}

pub fn _draw(map: &Vec<Vec<MapValue>>) {
    for row in map.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!();
    }
}

pub fn drop_sand(sand_drop: Point, map: &mut Vec<Vec<MapValue>>, to_drop: bool) -> bool {
    if to_drop {
        if map[sand_drop.y as usize][sand_drop.x as usize] != Empty {
            return false;
        }
    }
    let height = map.len() as i32 - 1;
    let width = map[0].len() as i32 - 1;

    let mut cur_point = sand_drop.clone();

    'search: loop {
        let directions = [
            cur_point.bottom(),
            cur_point.bottom().left(),
            cur_point.bottom().right(),
        ];
        for point in directions {
            if point.x < 0 || point.x > width || point.y < 0 || point.y > height {
                return false;
            }
            let value = map[point.y as usize][point.x as usize];
            if value == Empty {
                cur_point = point;
                continue 'search;
            }
        }
        map[cur_point.y as usize][cur_point.x as usize] = Sand;
        return true;
    }
}
