use std::ops::{Add, Div, Mul};

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

    /// Calculates the cross product.
    pub fn cross(self, other: Vector) -> Vector {
        let [x1, x2, x3] = self.0;
        let [y1, y2, y3] = other.0;
        Vector([x2 * y3 - x3 * y2, x3 * y1 - x1 * y3, x1 * y2 - x2 - y1])
    }

    /// Calculates the Euclidean norm.
    pub fn norm(self) -> f64 {
        self.0.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    /// Normalizes the `Vector`.
    pub fn normalize(self) -> Vector {
        self / self.norm()
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0.zip(rhs.0).map(|(x, y)| x + y))
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(rhs.0.map(|x| self * x))
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector(self.0.map(|x| x / rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_dot() {
        assert_eq!(Vector::new([1.0; 3]).dot(Vector::new([2.0; 3])), 6.0)
    }

    #[test]
    fn calculate_cross() {
        assert_eq!(
            Vector::new([1.0, 0.0, 0.0]).cross(Vector::new([0.0, 1.0, 0.0])),
            Vector::new([0.0, 0.0, 1.0])
        );
    }

    #[test]
    fn calculate_norm() {
        assert_eq!(Vector::new([1.0, 2.0, 2.0]).norm(), 3.0);
    }

    #[test]
    fn normalize_vector() {
        assert_eq!(
            Vector::new([1.0, 2.0, 2.0]).normalize(),
            Vector::new([1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0])
        )
    }

    #[test]
    fn add_vector() {
        assert_eq!(
            Vector([1.0, 2.0, 3.0]) + Vector([1.0, 2.0, 3.0]),
            Vector([2.0, 4.0, 6.0])
        );
    }

    #[test]
    fn add_identity() {
        assert_eq!(
            Vector([1.0, 2.0, 3.0]) + Vector([0.0; 3]),
            Vector([1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn multiply_by_scalar() {
        assert_eq!(
            2.0 * Vector::new([1.0, 2.0, 3.0]),
            Vector::new([2.0, 4.0, 6.0])
        );
    }

    #[test]
    fn multiply_by_identity() {
        assert_eq!(
            1.0 * Vector::new([1.0, 2.0, 3.0]),
            Vector::new([1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn divide_by_scalar() {
        assert_eq!(
            Vector::new([2.0, 4.0, 6.0]) / 2.0,
            Vector::new([1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn divide_by_identity() {
        assert_eq!(
            Vector::new([1.0, 2.0, 3.0]) / 1.0,
            Vector::new([1.0, 2.0, 3.0])
        );
    }
}
