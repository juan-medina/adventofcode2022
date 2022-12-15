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
use crate::point::Point;
use crate::ranges::Range;
use crate::sensor::Sensor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    fn new(sensors: Vec<Sensor>) -> Self {
        Self { sensors }
    }

    pub fn get_frequency_hidden_beacon(&self, max: i32) -> i64 {
        for row in 0..max {
            let (found, hidden) = self.get_no_cover_positions(row);
            if found {
                let multiplier: i64 = 4000000;
                let (x, y): (i64, i64) = (hidden.x as i64, hidden.y as i64);
                return x * multiplier + y;
            }
        }

        0
    }
    fn get_no_cover_positions(&self, row: i32) -> (bool, Point) {
        let mut found: bool = false;
        let mut hidden: Point = point::new(0, 0);
        let unmerged_visible = self.get_sensor_ranges_hitting_row(row);
        let merged = self.merge_ranges(&unmerged_visible);

        if merged.len() == 2 {
            found = true;
            hidden.x = merged[0].end + 1;
            hidden.y = row;
        }

        return (found, hidden);
    }

    pub fn positions_with_no_beacons(&self, row: i32) -> u32 {
        let sensor_ranges = self.get_sensor_ranges_hitting_row(row);
        let merged_ranges = self.merge_ranges(&sensor_ranges);

        let beacons = self.get_beacons_in_ranges(&merged_ranges, row);

        self.sum_size_of_ranges(&merged_ranges) - beacons.len() as u32
    }

    fn get_sensor_ranges_hitting_row(&self, row: i32) -> Vec<Range> {
        let mut ranges: Vec<Range> = vec![];

        for sensor in self.sensors.iter() {
            let (visible, range) = sensor.get_range_in_row(row);
            if visible {
                ranges.push(range);
            }
        }

        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        return ranges;
    }

    fn merge_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let mut merged_ranges: Vec<Range> = vec![];
        let mut merged_range: Range = Range::default();
        for i in 0..ranges.len() {
            let range = ranges[i];

            if i == 0 {
                merged_range = range;
                continue;
            }

            if merged_range.touches(range.start) {
                if range.end > merged_range.end {
                    merged_range.end = range.end;
                }
                continue;
            }

            merged_ranges.push(merged_range);
            merged_range = range;
        }
        merged_ranges.push(merged_range);

        return merged_ranges;
    }

    fn get_beacons_in_ranges(&self, ranges: &Vec<Range>, row: i32) -> Vec<i32> {
        let mut beacons: Vec<i32> = vec![];

        for reading in self.sensors.clone() {
            for range in ranges {
                if reading.closest_beacon.y == row {
                    let beacon = reading.closest_beacon.x;
                    if range.contains(beacon) && !beacons.contains(&beacon) {
                        beacons.push(beacon);
                    }
                }
            }
        }

        return beacons.clone();
    }

    fn sum_size_of_ranges(&self, ranges: &Vec<Range>) -> u32 {
        let mut total_size = 0;

        for r in ranges {
            total_size += r.size();
        }

        return total_size;
    }
}

pub fn new(sensors: Vec<Sensor>) -> Map {
    Map::new(sensors)
}
