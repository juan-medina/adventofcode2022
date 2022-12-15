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

use crate::point;
use crate::sensor::Sensor;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    sensors: Vec<Sensor>,
    lines: HashMap<i32, Vec<Vec<i32>>>,
}

impl Map {
    fn new(sensors: Vec<Sensor>) -> Self {
        let lines = HashMap::new();
        Self { sensors, lines }
    }

    fn fill_sensor(&mut self, sensor: &Sensor, min_row: i32, max_row: i32) {
        let dx = (sensor.at.x - sensor.closest_beacon.x).abs();
        let dy = (sensor.at.y - sensor.closest_beacon.y).abs();

        let point_top = point::new(sensor.at.x, sensor.at.y - dx - dy);
        let point_bottom = point::new(sensor.at.x, sensor.at.y + dx + dy);

        for y in point_top.y..point_bottom.y + 1 {
            let len = if y < sensor.at.y {
                y - point_top.y
            } else {
                point_bottom.y - y
            };

            if y >= min_row && y <= max_row {
                let range: Vec<i32> = vec![sensor.at.x - len, sensor.at.x + len];
                self.lines.entry(y).or_insert(Vec::new()).push(range);
            }
        }
    }

    pub fn fill(&mut self, min_row: i32, max_row: i32) {
        for sensor in self.sensors.clone() {
            self.fill_sensor(&sensor, min_row, max_row);
        }

        for (key, value) in self.lines.clone().iter() {
            *self.lines.get_mut(key).unwrap() = merge_overlapping(&value);
        }
    }

    pub fn get_blocked_at_row(&self, row: i32) -> usize {
        let mut total = 0usize;
        let ranges = self.lines.get(&row).unwrap().clone();
        for range in ranges {
            let begin = range[0];
            let end = range[1];
            total += (end - begin) as usize;
        }

        total
    }
}

pub fn new(sensors: Vec<Sensor>) -> Map {
    Map::new(sensors)
}

fn merge_overlapping(vec: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut clone = vec.clone();
    clone.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(clone[0].clone());

    for i in 1..clone.len() {
        let current: Vec<i32> = clone[i].clone();
        let j: usize = result.len() - 1;

        if current[0] >= result[j][0] && current[0] <= result[j][1] {
            result[j][1] = cmp::max(current[1], result[j][1]);
        } else {
            result.push(current);
        }
    }
    result
}
