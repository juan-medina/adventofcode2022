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

use crate::map;
use std::cmp::{max, min};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn left(&self) -> Point {
        let mut value = self.clone();
        value.x -= 1;
        value
    }
    pub fn right(&self) -> Point {
        let mut value = self.clone();
        value.x += 1;
        value
    }
    pub fn bottom(&self) -> Point {
        let mut value = self.clone();
        value.y += 1;
        value
    }
}

pub type Points = Vec<Point>;

pub fn normalize(points: &mut Vec<Vec<Point>>, extended: bool) -> (i32, Point) {
    let items: Vec<&mut Point> = points.into_iter().flatten().collect();

    let mut min_x = map::DROP_X;
    let mut max_x = map::DROP_X;
    let mut min_y = map::DROP_Y;
    let mut max_y = map::DROP_Y;

    for point in items {
        min_x = min(point.x, min_x);
        max_x = max(point.x, max_x);
        min_y = min(point.y, min_y);
        max_y = max(point.y, max_y);
    }

    if extended {
        max_y += 2;
    }

    let height = max_y - min_y + 1;

    if extended {
        min_x = min(min_x, map::DROP_X - height as i32);
        max_x = max(max_x, map::DROP_X + height as i32);
    }

    let width = max_x - min_x + 1;

    let reduce_x = min_x;

    for i in 0..points.len() {
        for j in 0..points[i].len() {
            points[i][j].x -= reduce_x;
        }
    }
    (reduce_x, Point::new(width, height))
}
