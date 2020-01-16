use cgmath::Vector2;

fn edgeFn<T>(a: Vector2<T>, b: Vector2<T>, p: Vector2<T>) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    // the 3 vertices should be counter clockwise
    return (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
}
