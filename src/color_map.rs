use sdl2::pixels::Color;

pub struct ColorMap {
    colors: Vec<Color>,
    positions: Vec<f64>,
}

impl ColorMap {
    pub fn new(colors: Vec<Color>, positions: Vec<f64>) -> Self {
        return ColorMap { colors, positions };
    }

    pub fn new_default() -> Self {
        return ColorMap {
            colors: vec![
                Color::RGB(0, 7, 100),
                Color::RGB(32, 107, 203),
                Color::RGB(237, 255, 255),
                Color::RGB(255, 170, 0),
                Color::RGB(0, 2, 0),
                Color::RGB(0, 0, 100),
            ],
            positions: vec![0.0, 0.16, 0.42, 0.6425, 0.8575, 1.0],
        };
    }

    pub fn val_to_lower_bound(&self, val: f64, max_val: f64) -> i32 {
        let pos_norm: f64 = val / max_val;
        let mut i = 0;
        while i < self.positions.len() && pos_norm >= self.positions[i] {
            i += 1;
        }
        i -= 1;
        return i as i32;
    }

    pub fn get_color(&self, val: f64, max_val: f64) -> Color {
        // between which two positions is val?
        let i = self.val_to_lower_bound(val, max_val);

        if val >= max_val {
            return Color::RGB(0, 0, 0);
        }

        if i >= self.colors.len() as i32 - 1 {
            return self.colors[self.colors.len() - 1];
        }

        let c1 = self.colors[i as usize];
        let c2 = self.colors[(i + 1) as usize];

        let norm_pos = (val / max_val - self.positions[i as usize])
            / (self.positions[(i + 1) as usize] - self.positions[i as usize]);

        // cubic interpolation between the two colors
        let r = (c1.r as f64 + norm_pos * (c2.r as f64 - c1.r as f64)) as u8;
        let g = (c1.g as f64 + norm_pos * (c2.g as f64 - c1.g as f64)) as u8;
        let b = (c1.b as f64 + norm_pos * (c2.b as f64 - c1.b as f64)) as u8;

        return Color::RGB(r, g, b);
    }
}
