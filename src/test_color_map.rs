#[cfg(test)]
use crate::color_map::ColorMap;

//const COLORS: Vec<Color> = vec![
//    Color::RGB(0, 7, 100),
//    Color::RGB(32, 107, 203),
//    Color::RGB(237, 255, 255),
//    Color::RGB(255, 170, 0),
//    Color::RGB(0, 2, 0),
//    Color::RGB(0, 0, 100),
//];
//const POSITIONS: Vec<f64> = vec![0.0, 0.16, 0.42, 0.6425, 0.8575, 1.0];
#[test]
fn test_color_map() {
    let color_map = ColorMap::new_default();

    assert_eq!(color_map.val_to_lower_bound(0.0, 100.0), 0);
    assert_eq!(color_map.val_to_lower_bound(20.0, 100.0), 1);
    assert_eq!(color_map.val_to_lower_bound(45.0, 100.0), 2);
    assert_eq!(color_map.val_to_lower_bound(65.0, 100.0), 3);
    assert_eq!(color_map.val_to_lower_bound(88.0, 100.0), 4);
    assert_eq!(color_map.val_to_lower_bound(100.0, 100.0), 5);
}
