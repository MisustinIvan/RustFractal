use crate::complex::Complex;
#[allow(clippy::redundant_field_names)]
use rayon::prelude::*;
use sdl2::pixels::Color;

#[derive(Clone, Debug)]
pub struct MandelbrotSet {
    pub width: u32,
    pub height: u32,
    pub max_iter: u32,
    pub canvas: Vec<Vec<u16>>,
    pub seed: Complex,
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl MandelbrotSet {
    pub fn new(w: u32, h: u32, max_iter: u32) -> Self {
        let mut canvas: Vec<Vec<u16>> = Vec::new();
        for _ in 0..h {
            let mut row: Vec<u16> = Vec::new();
            for _ in 0..w {
                row.push(0);
            }
            canvas.push(row);
        }

        return MandelbrotSet {
            seed: Complex::new(0.0, 0.0),
            width: w,
            height: h,
            left: -2.0,
            right: 1.0,
            top: 1.0,
            bottom: -1.0,
            max_iter,
            canvas,
        };
    }

    pub fn val_to_color_basic(&self, val: u16) -> Color {
        if val as u32 == self.max_iter {
            return Color::RGB(0, 0, 0);
        } else {
            let c = (val as u32 * 255 / self.max_iter) as u8;
            return Color::RGB(c, c, c);
        }
    }

    pub fn calculate(&mut self) {
        let mut new_canvas = self.canvas.clone();
        let width = self.width;
        let max_iter = self.max_iter;
        let height = self.height;
        new_canvas.par_iter_mut().enumerate().for_each(|(h, row)| {
            for w in 0..row.len() {
                let c = Complex::new(
                    (self.right - self.left) * (w as f64) / (width as f64) + self.left,
                    (self.top - self.bottom) * (h as f64) / (height as f64) + self.bottom,
                );
                let p = self.calculate_point(c, max_iter) as u16;
                *row.get_mut(w).unwrap() = p;
            }
        });
        self.canvas = new_canvas;
    }

    pub fn calculate_point(&self, c: Complex, max_iter: u32) -> u32 {
        //let mut z = Complex::new(0.0, 0.0);
        let mut z = self.seed;
        for i in 0..max_iter {
            z = z.abs();
            z = z * z + c;
            if z.mag() > 2.0 {
                return i;
            }
        }
        return max_iter;
    }

    pub fn zoom_in(&mut self, zoom: f64) {
        //              <width in coordinate space>    <width in coordinate space * zoom>
        let diff_width = ((self.right - self.left) - ((self.right - self.left) / zoom)) / 2.0;
        let diff_height = ((self.top - self.bottom) - ((self.top - self.bottom) / zoom)) / 2.0;

        self.left += diff_width;
        self.right -= diff_width;
        self.top -= diff_height;
        self.bottom += diff_height;
    }
    pub fn zoom_out(&mut self, zoom: f64) {
        let diff_width = (((self.right - self.left) * zoom) - (self.right - self.left)) / 2.0;
        let diff_height = (((self.top - self.bottom) * zoom) - (self.top - self.bottom)) / 2.0;

        self.left -= diff_width;
        self.right += diff_width;
        self.top += diff_height;
        self.bottom -= diff_height;
    }

    pub fn move_x(&mut self, x: f64) {
        let diff = (self.right - self.left) * x;
        self.left += diff;
        self.right += diff;
    }

    pub fn move_y(&mut self, y: f64) {
        let diff = (self.top - self.bottom) * y;
        self.top += diff;
        self.bottom += diff;
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        let x = (self.right - self.left) * (x as f64) / (self.width as f64) + self.left;
        let y = (self.top - self.bottom) * (y as f64) / (self.height as f64) + self.bottom;
        let diff_x = (self.right - self.left) / 2.0;
        let diff_y = (self.top - self.bottom) / 2.0;
        self.left = x - diff_x;
        self.right = x + diff_x;
        self.top = y + diff_y;
        self.bottom = y - diff_y;
    }
}
