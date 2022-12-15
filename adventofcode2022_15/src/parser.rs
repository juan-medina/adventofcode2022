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
use adventofcode2022_lib::utils::read_file;
use crate::sensor;
use crate::sensor::Sensor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser {
    file_name: String,
}

impl Parser {
    fn new(file_name: &str) -> Self {
        Self { file_name: String::from(file_name) }
    }
    pub fn parse(self) -> Vec<Sensor> {
        let mut sensors: Vec<Sensor> = Vec::new();

        let re_parse = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

        for line in read_file(self.file_name.as_str()) {
            let caps = re_parse.captures(&line).unwrap();

            let at_x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let at_y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_x = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_y = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
            sensors.push(sensor::new(at_x, at_y, beacon_x, beacon_y));
        }
        sensors
    }
}

pub fn new(file_name: &str) -> Parser {
    Parser::new(file_name)
}