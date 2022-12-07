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
        fmt,
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader, Read},
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

    // read a file in lines
    pub fn read_file(filename: &str) -> Vec<String> {
        // the file lines
        let mut lines: Vec<String> = Vec::new();

        //open the file
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        // read the file line by line
        for (_index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            lines.push(line);
        }
        lines
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

    #[derive(Clone, Copy)]
    pub enum RunType {
        Part1 = 1,
        Part2 = 2,
    }

    #[derive(Clone, Copy)]
    pub enum FileType {
        ExampleFile,
        PuzzleFile,
    }

    impl fmt::Display for FileType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                FileType::ExampleFile => write!(f, "example"),
                FileType::PuzzleFile => write!(f, "puzzle"),
            }
        }
    }

    type RunFunction<T> = fn(&str, RunType) -> T;

    pub struct Example<T: Any + Display> {
        number: u32,
        name: String,
        output: String,
        file: String,
        func: RunFunction<T>,
    }

    impl<T: Any + Display> Example<T> {
        pub fn new(
            number: u32,
            name: &str,
            output: &str,
            file: &str,
            func: RunFunction<T>,
        ) -> Example<T> {
            Example {
                number: number,
                name: name.to_string(),
                output: output.to_string(),
                file: file.to_string(),
                func: func,
            }
        }

        pub fn run_part(&self, file_type: FileType, run_type: RunType) -> T {
            let file_name = format!("data/{}_{}.dat", self.file, file_type);
            (self.func)(&file_name, run_type)
        }

        pub fn run_all(&self) {
            println!("Advent of Code 2022 - Day {}: {}", self.number, self.name);
            println!();

            for part in [RunType::Part1, RunType::Part2] {
                let part_num = part.clone() as i32;
                for file_type in [FileType::ExampleFile, FileType::PuzzleFile] {
                    let label = format!("part {part_num} [{file_type}]");
                    print_result(&label, &self.output, self.run_part(file_type, part));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::*;

    #[test]
    fn test_has_duplicates() {
        assert_eq!(true, has_duplicates(&[1, 2, 1]));
        assert_eq!(false, has_duplicates(&[1, 2, 3]));
        assert_eq!(true, has_duplicates(&["a", "b", "a"]));
        assert_eq!(false, has_duplicates(&["a", "b", "c"]));
    }

    #[test]
    fn test_read_whole_file() {
        assert_eq!(
            "this is a test file",
            read_whole_file("data/test_example.dat")
        );
    }

    #[test]
    fn test_read_file() {
        assert_eq!(5, read_file("data/test_puzzle.dat").len());
    }

    fn count_lines_or_characters(filename: &str, run_type: RunType) -> usize {
        match run_type {
            RunType::Part1 => {
                let lines = read_file(filename);
                lines.len()
            }
            RunType::Part2 => {
                let content = read_whole_file(filename);
                content.len()
            }
        }
    }

    #[test]
    fn test_run_all() {
        let example = Example::new(1, "super fun", "total", "test", count_lines_or_characters);
        example.run_all();
    }

    #[test]
    fn test_run_part() {
        let example = Example::new(1, "super fun", "total", "test", count_lines_or_characters);
        assert_eq!(1, example.run_part(FileType::ExampleFile, RunType::Part1));
        assert_eq!(19, example.run_part(FileType::ExampleFile, RunType::Part2));
    }
}
