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

use crate::map::MapValue;
use crate::map::MapValue::{Empty, Rock, Sand};
use image::imageops::FilterType;
use crate::points::Point;

#[derive(Clone, Copy)]
pub struct Draw {
    _frame: u32,
}

impl Draw {
    pub fn new() -> Self {
        Draw { _frame: 0 }
    }
    pub fn frame(&mut self, _: &Vec<Vec<MapValue>>, _: &Point) {

    }
    pub fn _frame(&mut self, map: &Vec<Vec<MapValue>>, searching_at: &Point) {
        let height = map.len() as u32;
        let width = map[0].len() as u32;

        let mut rgb_image = image::RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let val = map[y as usize][x as usize];
                let color = match val {
                    Empty => image::Rgb([0, 0, 0]),
                    Rock => image::Rgb([90, 77, 65]),
                    Sand => image::Rgb([242, 209, 107]),
                };

                rgb_image.put_pixel(x, y, color);
            }
        }

        if searching_at.x != 0 && searching_at.y != 0 {
            rgb_image.put_pixel(searching_at.x as u32, searching_at.y as u32, image::Rgb([255, 255, 255]));
        }

        let scale_x = 1920 / width;

        let scaled = image::imageops::resize(&rgb_image, width * scale_x, height * scale_x, FilterType::Nearest);

        self._frame += 1;
        let filename = format!("target/frame_{:010}.png", self._frame);

        scaled.save(filename).unwrap();
    }
}

