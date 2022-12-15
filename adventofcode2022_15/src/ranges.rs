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

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}
impl Default for Range {
    fn default() -> Range {
        Range { start: 0, end: 0 }
    }
}
impl Range {
    pub fn contains(&self, location: i32) -> bool {
        return location >= self.start && location <= self.end;
    }
    pub fn touches(&self, location: i32) -> bool {
        return location >= self.start - 1 && location <= self.end + 1;
    }
    pub fn size(&self) -> u32 {
        return (self.start - self.end).abs() as u32 + 1u32;
    }
}

pub fn new() -> Range {
    Range::default()
}
