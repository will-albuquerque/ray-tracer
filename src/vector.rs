/// A three dimensional vector.
#[derive(Clone, Copy)]
pub struct Vector([f64; 3]);

impl Vector {
    /// Creates a new `Vector`.
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector([x, y, z])
    }
}
