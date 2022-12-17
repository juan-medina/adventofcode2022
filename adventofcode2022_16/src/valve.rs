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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valve {
    pub name: String,
    pub rate: u32,
    pub tunnels_indexes: Vec<usize>,
    pub index: usize,
}

impl Valve {
    pub fn index_from_name(name: &str) -> usize {
        let chars = name.as_bytes();
        (chars[0] - b'A') as usize * 26 + (chars[1] - b'A') as usize
    }

    fn new(name: &str, rate: u32) -> Self {
        Self {
            name: String::from(name),
            index: Self::index_from_name(name),
            tunnels_indexes: Vec::new(),
            rate: rate,
        }
    }

    pub fn add_tunnel(&mut self, index: usize) {
        self.tunnels_indexes.push(index);
    }
}

pub fn new(name: &str, rate: u32) -> Valve {
    Valve::new(name, rate)
}
