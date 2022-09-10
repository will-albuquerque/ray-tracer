use crate::{Point, Vector};

/// A three dimensional ray.
#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    /// Creates a new `Ray`.
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /// Returns the `Ray`'s origin.
    pub fn origin(self) -> Point {
        self.origin
    }

    /// Returns the `Ray`'s direction.
    pub fn direction(self) -> Vector {
        self.direction
    }

    /// Computes the `Point` at `t`.
    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_point() {
        let ray = Ray::new(Point::new([1.0, 0.0, 0.0]), Vector::new([1.0, 2.0, 3.0]));
        assert_eq!(ray.at(2.0), Point::new([3.0, 4.0, 6.0]));
    }
}
