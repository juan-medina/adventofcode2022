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

use crate::point::Point;
use crate::ranges::Range;
use crate::{point, ranges};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sensor {
    pub at: Point,
    pub closest_beacon: Point,
}

impl Sensor {
    pub fn get_range_in_row(&self, row: i32) -> (bool, Range) {
        let mut range: Range = ranges::new();

        let overlap = self.get_sensor_radius_on_row(row);
        let visible = if overlap < 0 { false } else { true };

        range.start = if visible { self.at.x - overlap } else { 0 };
        range.end = if visible { self.at.x + overlap } else { 0 };

        return (visible, range);
    }

    fn get_sensor_radius_on_row(&self, row: i32) -> i32 {
        let distance = self.distance() as i32;

        let row_delta = (self.at.y - row).abs();

        return distance - row_delta;
    }

    fn distance(&self) -> u32 {
        let x_delta = (self.at.x - self.closest_beacon.x).abs() as u32;
        let y_delta = (self.at.y - self.closest_beacon.y).abs() as u32;
        return x_delta + y_delta;
    }
}

impl Sensor {
    fn new(at: Point, closest_beacon: Point) -> Self {
        Self { at, closest_beacon }
    }
}

pub fn new(x: i32, y: i32, beacon_x: i32, beacon_y: i32) -> Sensor {
    Sensor::new(point::new(x, y), point::new(beacon_x, beacon_y))
}
