use sdl2::pixels::Color;

pub struct ColorMap {
    colors: Vec<Color>,
    positions: Vec<f64>,
}

impl ColorMap {
    pub fn new() -> Self {
        return ColorMap {
            colors: vec![
                Color::RGB(0, 7, 100),
                Color::RGB(32, 107, 203),
                Color::RGB(237, 255, 255),
                Color::RGB(255, 170, 0),
                Color::RGB(0, 2, 0),
            ],
            positions: vec![0.0, 0.16, 0.42, 0.6425, 0.8575],
        };
    }

    pub fn get_color(&self, val: f64, max_val: f64) -> i32 {
        // between which two positions is val?
        let pos_norm: f64 = val / max_val;
        let mut i = 0;
        while i < self.positions.len() && pos_norm >= self.positions[i] {
            i += 1;
        }
        i -= 1;

        return i as i32;
    }
}

#[test]
fn test_color_map() {
    let color_map = ColorMap::new();

    assert_eq!(color_map.get_color(0.0, 100.0), 0);
    assert_eq!(color_map.get_color(20.0, 100.0), 1);
    assert_eq!(color_map.get_color(45.0, 100.0), 2);
    assert_eq!(color_map.get_color(65.0, 100.0), 3);
    assert_eq!(color_map.get_color(88.0, 100.0), 4);
}
