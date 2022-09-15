use image::Rgb;

use crate::{Point, Ray, Vector};

/// A three dimensional sphere.
#[derive(Clone, Copy)]
pub struct Sphere {
    center: Point,
    radius: f64,
    color: Rgb<f32>,
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn new(center: Point, radius: f64, color: Rgb<f32>) -> Sphere {
        Sphere {
            center,
            radius,
            color,
        }
    }

    /// Returns the `Sphere`'s color.
    pub fn color(self) -> Rgb<f32> {
        self.color
    }

    /// Casts a `Ray` at `self`.
    /// Returns a `Hit` if the `Ray` hits `self`.
    pub fn cast(self, ray: Ray) -> Option<Hit> {
        // Rename varialbes
        let c = self.center;
        let r = self.radius;
        let o = ray.origin();
        let d = ray.direction();

        // Solve the equation: (d.d)t^2 + 2t(u.(o-c)) + (o-c).(o-c) - r^2 = 0
        // Find smallest positive value of t
        let x = d.dot(d);
        let y = d.dot(o - c);
        let z = (o - c).dot(o - c) - r.powi(2);

        let del = y.powi(2) - x * z;
        if del < 0.0 {
            // Ray does not intersect with sphere
            return None;
        }

        let t = if (-y - del.sqrt()) / x > 0.0 {
            (-y - del.sqrt()) / x
        } else if (-y + del.sqrt()) / x > 0.0 {
            (-y + del.sqrt()) / x
        } else {
            return None;
        };

        Some(Hit {
            t,
            normal: ray.at(t) - c,
        })
    }
}

/// Information about how a `Ray` hit a `Sphere`.
#[derive(Clone, Copy)]
pub struct Hit {
    t: f64,
    normal: Vector,
}

impl Hit {
    /// Returns how far along the ray the intersection occured.
    pub fn t(self) -> f64 {
        self.t
    }

    /// Returns the normal vector at the point of intersection.
    /// The normal vector always points away from the surface.
    pub fn normal(self) -> Vector {
        self.normal
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector;

    use super::*;

    fn unit_sphere() -> Sphere {
        Sphere::new(Point::new([0.0; 3]), 1.0, Rgb([0.0; 3]))
    }

    #[test]
    fn hit_outside() {
        let ray = Ray::new(Point::new([-2.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = unit_sphere();
        let hit = sphere.cast(ray).unwrap();
        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.normal, Vector::new([-1.0, 0.0, 0.0]));
    }

    #[test]
    fn hit_inside() {
        let ray = Ray::new(Point::new([0.0; 3]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = unit_sphere();
        let hit = sphere.cast(ray).unwrap();
        assert_eq!(hit.t, 1.0);
        assert_eq!(hit.normal, Vector::new([1.0, 0.0, 0.0]));
    }

    #[test]
    fn skim_surface() {
        let ray = Ray::new(Point::new([-2.0, 1.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = unit_sphere();
        let hit = sphere.cast(ray).unwrap();
        assert_eq!(hit.t, 2.0);
        assert_eq!(hit.normal, Vector::new([0.0, 1.0, 0.0]));
    }

    #[test]
    fn sphere_behind_ray() {
        let ray = Ray::new(Point::new([1.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        let sphere = unit_sphere();
        assert!(sphere.cast(ray).is_none());
    }

    #[test]
    fn miss() {
        let ray = Ray::new(Point::new([-2.0, 0.0, 0.0]), Vector::new([-1.0, 0.0, 0.0]));
        let sphere = unit_sphere();
        assert!(sphere.cast(ray).is_none());
    }
}
