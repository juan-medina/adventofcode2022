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

use crate::valve;
use crate::valve::Valve;
use adventofcode2022_lib::utils::read_file;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser {
    file_name: String,
}

impl Parser {
    fn new(file_name: &str) -> Self {
        Self {
            file_name: String::from(file_name),
        }
    }
    pub fn parse(self) -> Vec<Valve> {
        let mut valves: Vec<Valve> = Vec::new();

        let re_parse = Regex::new(r"(?m)Valve ([A-Z]{2}).+rate=(\d+);.+valves? (.+)").unwrap();
        let re_leads = Regex::new(r"(?m)([A-Z]{2})").unwrap();

        for line in read_file(self.file_name.as_str()) {
            let caps = re_parse.captures(&line).unwrap();

            let name = caps.get(1).unwrap().as_str();
            let rate = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let leads = caps.get(3).unwrap().as_str();

            let mut valve = valve::new(name, rate);

            let caps = re_leads.captures_iter(leads);
            for item in caps {
                valve.add_tunnel(Valve::index_from_name(&item[0]));
            }
            valves.push(valve);
        }
        valves
    }
}

pub fn new(file_name: &str) -> Parser {
    Parser::new(file_name)
}
