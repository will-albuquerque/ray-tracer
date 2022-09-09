use crate::{point::Point, vector::Vector};

/// A three dimensional ray.
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}
