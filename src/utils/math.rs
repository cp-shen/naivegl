/// if the 3 vertices are counter-clockwise
/// returns positive value when point p is on the left side of line ab
/// if the 3 vertices are colinear
/// returns 0
pub fn edge_fn<T>(a: (T, T), b: (T, T), p: (T, T)) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    return (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
}
