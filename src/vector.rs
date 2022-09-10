use std::ops::Mul;

/// A three dimensional vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector([f64; 3]);

impl Vector {
    /// Creates a new `Vector`.
    pub fn new(inner: [f64; 3]) -> Vector {
        Vector(inner)
    }

    /// Unwraps this `Vector`, returning the underlying array.
    pub fn into_inner(self) -> [f64; 3] {
        self.0
    }

    /// Calculates the dot product.
    pub fn dot(self, other: Vector) -> f64 {
        self.0.zip(other.0).map(|(x, y)| x * y).iter().sum()
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
        assert_eq!(
            2.0 * Vector::new([1.0, 2.0, 3.0]),
            Vector::new([2.0, 4.0, 6.0])
        );
    }

    #[test]
    fn scale_by_identity() {
        assert_eq!(
            1.0 * Vector::new([1.0, 2.0, 3.0]),
            Vector::new([1.0, 2.0, 3.0])
        );
    }
}
