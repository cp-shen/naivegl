pub fn get_8bit_color(f: f64) -> u8 {
    assert_eq!(f >= 0.0, true);
    assert_eq!(f <= 1.0, true);
    (f * 255.0).round() as u8
}
