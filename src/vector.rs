/// A three dimensional vector.
#[derive(Clone, Copy)]
pub struct Vector([f64; 3]);

impl Vector {
    /// Creates a new `Vector`.
    pub fn new(inner: [f64; 3]) -> Vector {
        Vector(inner)
    }
}
