use std::ops::{Add, Sub};

use crate::Vector;

/// A three dimensional point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point([f64; 3]);

impl Point {
    /// Creates a new `Point`.
    pub fn new(inner: [f64; 3]) -> Point {
        Point(inner)
    }

    /// Unwraps this `Point`, returning the underlying array.
    pub fn into_inner(self) -> [f64; 3] {
        self.0
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point(self.0.zip(rhs.into_inner()).map(|(x, y)| x + y))
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.0.zip(rhs.0).map(|(x, y)| x - y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displace() {
        assert_eq!(
            Point::new([0.0, 0.0, 0.0]) + Vector::new([1.0, 2.0, 3.0]),
            Point::new([1.0, 2.0, 3.0])
        );
    }

    #[test]
    fn displace_by_identity() {
        assert_eq!(
            Point::new([0.0, 0.0, 0.0]) + Vector::new([0.0, 0.0, 0.0]),
            Point::new([0.0, 0.0, 0.0])
        );
    }
}
