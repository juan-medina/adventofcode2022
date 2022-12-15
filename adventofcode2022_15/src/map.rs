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
use std::fmt;
use crate::point;
use crate::point::Point;
use crate::sensor::Sensor;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Empty,
    Beacon,
    Sensor,
    Blocked,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Empty => write!(f, "."),
            Value::Beacon => write!(f, "B"),
            Value::Sensor => write!(f, "S"),
            Value::Blocked => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    sensors: Vec<Sensor>,
    data: Vec<Vec<Value>>,
    width: usize,
    height: usize,
    shift: Point,
}

impl Map {
    fn new(sensors: Vec<Sensor>) -> Self {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for sensor in sensors.clone() {
            let dx = (sensor.at.x - sensor.closest_beacon.x).abs();
            let dy = (sensor.at.y - sensor.closest_beacon.y).abs();
            min_x = min(sensor.at.x - (dx + dy), min_x);
            max_x = max(sensor.at.x + (dx + dy), max_x);
            min_y = min(sensor.at.y - (dx + dy), min_y);
            max_y = max(sensor.at.y + (dx + dy), max_y);
        }

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        let shift = point::new(min_x, min_y);

        let data: Vec<Vec<Value>> = vec![vec![Value::Empty; width]; height];

        Self { sensors, data, width, height, shift }
    }

    pub fn _draw(self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.data[y][x]);
            }
            println!();
        }
    }

    fn fill_sensor(&mut self, sensor: &Sensor) {
        let dx = (sensor.at.x - sensor.closest_beacon.x).abs();
        let dy = (sensor.at.y - sensor.closest_beacon.y).abs();

        let point_top = point::new(sensor.at.x, sensor.at.y - dx - dy);
        let point_bottom = point::new(sensor.at.x, sensor.at.y + dx + dy);

        let point_left = point::new(sensor.at.x - dx - dy, sensor.at.y);
        let point_right = point::new(sensor.at.x + dx + dy, sensor.at.y);

        self.set(&point_top, Value::Blocked);
        self.set(&point_bottom, Value::Blocked);

        self.set(&point_left, Value::Blocked);
        self.set(&point_right, Value::Blocked);


        for y in point_top.y..point_bottom.y + 1 {
            let len = if y < sensor.at.y {
                y - point_top.y
            } else {
                point_bottom.y - y
            };

            for x in sensor.at.x - len..sensor.at.x + len + 1 {
                let point = point::new(x, y);
                self.set(&point, Value::Blocked);
            }
        }
    }

    pub fn fill(&mut self) {
        for sensor in self.sensors.clone() {
            println!("filling sensor at {} {}", sensor.at.x, sensor.at.y);
            self.fill_sensor(&sensor);
        }
        for sensor in self.sensors.clone() {
            self.set(&sensor.at, Value::Sensor);
            self.set(&sensor.closest_beacon, Value::Beacon);
        }
    }

    fn set(&mut self, point: &Point, value: Value) {
        let x = (point.x - self.shift.x) as usize;
        let y = (point.y - self.shift.y) as usize;
        self.data[y][x] = value;
    }

    pub fn get_blocked_at_row(&self, row: i32) -> usize {
        let y = (row - self.shift.y) as usize;

        let filtered: Vec<&Value> = self.data[y].iter().filter(|x| {
            *x == &Value::Blocked
        }).collect();
        filtered.iter().count() as usize
    }
}

pub fn new(sensors: Vec<Sensor>) -> Map {
    Map::new(sensors)
}