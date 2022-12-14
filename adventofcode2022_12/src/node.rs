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

use crate::step::NavigationStep;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Node {
    pub x: usize,
    pub y: usize,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn step(&self, step_num: usize) -> NavigationStep {
        NavigationStep::new(self.clone(), step_num)
    }
    pub fn get_near_nodes(&self, width: usize, height: usize) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();

        if self.x > 0 {
            nodes.push(Node::new(self.x - 1, self.y));
        }
        if self.x < width - 1 {
            nodes.push(Node::new(self.x + 1, self.y));
        }
        if self.y > 0 {
            nodes.push(Node::new(self.x, self.y - 1));
        }
        if self.y < height - 1 {
            nodes.push(Node::new(self.x, self.y + 1));
        }

        nodes
    }
}
