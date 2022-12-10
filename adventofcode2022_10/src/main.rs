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

use adventofcode2022_lib::utils::{read_file, Example, RunType};

const NAME: &'static str = "Cathode-Ray Tube";
const OUTPUT: &'static str = "result";
const FILE: &'static str = "instructions";

fn main() {
    Example::new(10, NAME, OUTPUT, FILE, solve_day_10).run_all();
}

fn get_sprite(pos: i32) -> Vec<usize> {
    let start = pos - 1;

    let mut result = vec![0; 40];

    for x in 0..3 {
        let pixel_pos = start + x;
        if pixel_pos >= 0 && pixel_pos <= 39 {
            result[pixel_pos as usize] = 1;
        }
    }

    return result;
}

fn solve_day_10(filename: &str, run_type: RunType) -> String {
    let mut x_register: i32 = 1;
    let mut cycle: usize = 1;
    let mut total_strength: i32 = 0;
    let mut next_check = 20;

    let mut current_sprite = get_sprite(x_register);
    let mut crt: Vec<Vec<usize>> = Vec::new();

    let lines = read_file(filename);
    for line in lines {
        let cycles_to_do: usize;
        let value_to_add: i32;

        if line == "noop" {
            cycles_to_do = 1;
            value_to_add = 0;
        } else {
            cycles_to_do = 2;
            let (_, right) = line.split_once(" ").unwrap();
            value_to_add = right.parse::<i32>().unwrap();
        }

        for cycle_counter in 0..cycles_to_do {
            // begin of cycle

            // in cycle
            if run_type == RunType::Part1 {
                if cycle == next_check {
                    next_check += 40;
                    let strength = cycle as i32 * x_register;
                    total_strength += strength;
                }
            } else {
                draw_pixel(cycle, &current_sprite, &mut crt);
            }

            // end instruction
            if line != "noop" && cycle_counter == cycles_to_do - 1 {
                x_register += value_to_add;
                if run_type == RunType::Part2 {
                    current_sprite = get_sprite(x_register);
                }
            }

            // end of cycle

            // next cycle
            cycle += 1;
        }
    }

    match run_type {
        RunType::Part1 => total_strength.to_string(),
        RunType::Part2 => {
            let mut result: String = String::from("\n");
            for line in crt {
                for pixel in line {
                    if pixel == 1 {
                        result += "#";
                    } else {
                        result += ".";
                    }
                }
                result += "\n";
            }
            result
        }
    }
}

fn draw_pixel(pos: usize, sprite: &Vec<usize>, crt: &mut Vec<Vec<usize>>) {
    let pos: i32 = (pos as i32) - 1;

    let lines = crt.len() as i32;
    let line = pos.div_euclid(40);
    let col = pos - (line * 40);

    if line > (lines - 1) {
        crt.push(vec![0; 40]);
    }

    crt[line as usize][col as usize] = sprite[col as usize];
}

#[cfg(test)]
mod tests {
    use crate::*;
    use adventofcode2022_lib::utils::FileType;

    #[test]
    fn test_part_1() {
        let example = Example::new(10, NAME, OUTPUT, FILE, solve_day_10);
        assert_eq!(
            "13140",
            example.run_part(FileType::ExampleFile, RunType::Part1)
        );
    }

    #[test]
    fn test_part_2() {
        let example = Example::new(1, NAME, OUTPUT, FILE, solve_day_10);
        assert_eq!(
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
",
            example.run_part(FileType::ExampleFile, RunType::Part2)
        );
    }
}
