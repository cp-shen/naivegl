pub fn get_8bit_color(f: f64) -> u8 {
    if f < 0.0 {
        return 0;
    };

    if f > 1.0 {
        return 255;
    };

    (f * 255.0).round() as u8
}
