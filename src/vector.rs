/// A three dimensional vector.
pub struct Vector([f64; 3]);

impl Vector {
    /// Creates a new `Vector`.
    pub fn new(inner: [f64; 3]) -> Vector {
        Vector(inner)
    }
}
