use std::ops::Add;

use super::Vector;

/// A three dimensional point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point([f64; 3]);

impl Point {
    /// Creates a new `Point`.
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point([x, y, z])
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point(self.0.zip(rhs.0).map(|(x, y)| x + y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn displace() {
        assert_eq!(
            Point::new(0.0, 0.0, 0.0) + Vector::new(1.0, 2.0, 3.0),
            Point::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn displace_by_identity() {
        assert_eq!(
            Point::new(0.0, 0.0, 0.0) + Vector::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 0.0)
        );
    }
}
