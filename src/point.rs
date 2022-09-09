/// A three dimensional point
pub struct Point([f64; 3]);

impl Point {
    /// Creates a new `Point`.
    pub fn new(inner: [f64; 3]) -> Point {
        Point(inner)
    }
}
