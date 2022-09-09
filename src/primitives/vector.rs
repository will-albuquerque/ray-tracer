use std::ops::Mul;

/// A three dimensional vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector(pub(super) [f64; 3]);

impl Vector {
    /// Creates a new `Vector`.
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector([x, y, z])
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(rhs.0.map(|x| self * x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() {
        assert_eq!(2.0 * Vector::new(1.0, 2.0, 3.0), Vector::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn scale_by_identity() {
        assert_eq!(1.0 * Vector::new(1.0, 2.0, 3.0), Vector::new(1.0, 2.0, 3.0));
    }
}
