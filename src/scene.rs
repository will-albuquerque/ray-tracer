use std::f64::consts::PI;

use image::{Pixel, Rgb};

use rand;

use crate::{Ray, Sphere, Vector};

/// A renderable scene.
pub struct Scene {
    spheres: Vec<Sphere>,
}

impl Scene {
    /// Creates a `Scene`.
    pub fn new(spheres: &[Sphere]) -> Scene {
        Scene {
            spheres: spheres.to_vec(),
        }
    }

    /// Traces a `Ray` through `self`, returning its color.
    pub fn trace(&self, mut ray: Ray, max_bounces: u32) -> Rgb<f32> {
        let mut color = Rgb([1.0; 3]);
        for _ in 0..=max_bounces {
            let closest = self
                .spheres
                .iter()
                .filter_map(|sphere| sphere.cast(ray).map(|hit| (sphere, hit)))
                .min_by(|(_, x), (_, y)| x.t().total_cmp(&y.t()));

            match closest {
                Some((&sphere, hit)) => {
                    // Update color
                    color.apply2(&sphere.color(), |x, y| x * y);

                    // Calculate random unit vector
                    let theta = 2.0 * PI * rand::random::<f64>();
                    let phi = 2.0 * PI * rand::random::<f64>();
                    let random = Vector::new([
                        theta.sin() * phi.cos(),
                        theta.sin() * phi.sin(),
                        theta.cos(),
                    ]);

                    // Create ray
                    let origin = ray.at(hit.t());
                    let direction = random + hit.normal().normalize();
                    ray = Ray::new(origin, direction);
                }
                None => return color,
            };
        }
        Rgb([0.0; 3])
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::*;

    #[test]
    fn no_hits() {
        let scene = Scene::new(&[]);
        let ray = Ray::new(Point::new([0.0; 3]), Vector::new([1.0; 3]));
        assert_eq!(scene.trace(ray, 1), Rgb([1.0; 3]));
    }

    #[test]
    fn one_hit() {
        let scene = Scene::new(&[Sphere::new(Point::new([0.0; 3]), 1.0, Rgb([0.5; 3]))]);
        let ray = Ray::new(Point::new([-2.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        assert_eq!(scene.trace(ray, 1), Rgb([0.5; 3]));
    }

    #[test]
    fn one_behind() {
        let scene = Scene::new(&[
            Sphere::new(Point::new([0.0; 3]), 1.0, Rgb([1.0, 0.0, 0.0])),
            Sphere::new(Point::new([4.0, 0.0, 0.0]), 1.0, Rgb([0.0, 1.0, 0.0])),
        ]);
        let ray = Ray::new(Point::new([-2.0, 0.0, 0.0]), Vector::new([1.0, 0.0, 0.0]));
        assert_eq!(scene.trace(ray, 1), Rgb([1.0, 0.0, 0.0]));
    }
}
