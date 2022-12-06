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

pub mod utils {
    use std::{
        any::Any,
        fmt::Display,
        fs::File,
        io::{BufReader, Read},
    };

    pub fn print_result<T: Any + Display>(label: &str, name: &str, value: T) {
        println!("{label} {name}: {value}");
    }

    // check if we have duplicates in an slice
    pub fn has_duplicates<T: Any + PartialEq>(marker: &[T]) -> bool {
        let bound = marker.len();
        for i in 0..bound {
            for j in i + 1..bound {
                if marker[i] == marker[j] {
                    return true;
                }
            }
        }
        false
    }

    // read a whole file
    pub fn read_whole_file(filename: &str) -> String {
        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader
            .read_to_string(&mut contents)
            .expect(&format!("file not found: {filename}"));
        contents
    }
}
