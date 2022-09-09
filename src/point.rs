/// A three dimensional point.
#[derive(Clone, Copy)]
pub struct Point([f64; 3]);

impl Point {
    /// Creates a new `Point`.
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point([x, y, z])
    }
}
