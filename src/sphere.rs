use crate::{Point, Ray};

/// A three dimensional sphere.
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    /// Casts a `Ray` at `self`.
    /// Returns `t` if the `Ray` hits `self`.
    pub fn cast(self, ray: Ray) -> Option<f64> {
        // Rename varialbes
        let c = self.center;
        let r = self.radius;
        let o = ray.origin();
        let d = ray.direction();

        // Solve the equation: (d.d)t^2 + 2t(u.(o-c)) + (o-c).(o-c) - r^2 = 0
        // Find smallest positive value of t
        let a = d.dot(d);
        let b = d.dot(o - c);
        let c = (o - c).dot(o - c) - r.powi(2);

        let del = b.powi(2) - a * c;
        if del < 0.0 {
            // Ray does not intersect with sphere
            return None;
        }

        let t = (-b - del.sqrt()) / a;
        if t > 0.0 {
            return Some(t);
        }

        let t = (-b + del.sqrt()) / a;
        match t > 0.0 {
            true => Some(t),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    use super::*;

    #[test]
    fn hit_outside() {
        let ray = Ray::new(Point::new([-2.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = Sphere::new(Point::new([0.0; 3]), 1.0);
        assert_eq!(sphere.cast(ray), Some(1.0));
    }

    #[test]
    fn hit_inside() {
        let ray = Ray::new(Point::new([0.0; 3]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = Sphere::new(Point::new([0.0; 3]), 1.0);
        assert_eq!(sphere.cast(ray), Some(1.0));
    }

    #[test]
    fn skim_surface() {
        let ray = Ray::new(Point::new([-2.0, 1.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = Sphere::new(Point::new([0.0; 3]), 1.0);
        assert_eq!(sphere.cast(ray), Some(2.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::new(Point::new([1.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = Sphere::new(Point::new([0.0; 3]), 1.0);
        assert_eq!(sphere.cast(ray), None);
    }

    #[test]
    fn miss() {
        let ray = Ray::new(Point::new([-2.0, 0.0, 0.0]), Vector::new([-1.0, 0.0, 0.0]));
        let sphere = Sphere::new(Point::new([0.0; 3]), 1.0);
        assert_eq!(sphere.cast(ray), None);
    }
}
