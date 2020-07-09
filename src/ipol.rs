pub fn interpolate_linear(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    // println!("x: {}", x);
    // println!("x0: {}", x0);
    // println!("x1: {}", x1);
    // println!("y0: {}", y0);
    // println!("y1: {}", y1);
    let dx = x1 - x0;
    if dx.abs() <= std::f64::EPSILON {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / dx
}
