pub fn edgeFn<T>(a: (T, T), b: (T, T), p: (T, T)) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    // the 3 vertices should be counter clockwise
    return (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
}
