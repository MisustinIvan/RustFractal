#[cfg(test)]
use crate::color_map::ColorMap;
#[test]
fn test_color_map() {
    let color_map = ColorMap::new();

    assert_eq!(color_map.get_color(0.0, 100.0), 0);
    assert_eq!(color_map.get_color(20.0, 100.0), 1);
    assert_eq!(color_map.get_color(45.0, 100.0), 2);
    assert_eq!(color_map.get_color(65.0, 100.0), 3);
    assert_eq!(color_map.get_color(88.0, 100.0), 4);
}
