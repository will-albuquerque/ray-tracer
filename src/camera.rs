use std::iter;

use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;

use rand;

use image::{Pixel, Rgb32FImage};

use crate::{Point, Ray, Scene, Vector};

/// A `Camera` used to render a `Scene`.
pub struct Camera {
    width: u32,
    height: u32,
    eye: Point,
    i: Vector,
    j: Vector,
    k: Vector,
}

impl Camera {
    /// Creates a new `Camera`.
    pub fn new(
        width: u32,
        height: u32,
        vfov: f64,
        eye: Point,
        target: Point,
        up: Vector,
    ) -> Camera {
        // Find distance between eye and plane with dimensions width by height
        let d = height as f64 / 2.0 / (vfov / 2.0).tan();

        // Find j, the upward vector
        let j = up.normalize();

        // Find k, the forward vector
        let k = d * (target - eye).normalize();

        // Find i, the leftward vector
        let i = j.cross(k).normalize();

        Camera {
            width,
            height,
            eye,
            i,
            j,
            k,
        }
    }

    /// Renders a `Scene`.
    pub fn render(&self, scene: &Scene, samples: usize, max_bounces: u32) -> Rgb32FImage {
        let mut image = Rgb32FImage::new(self.width, self.height);
        image
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| {
                *pixel = iter::repeat_with(|| {
                    (self.width as f64 / 2.0 - x as f64 - rand::random::<f64>()) * self.i
                        + (self.height as f64 / 2.0 - y as f64 - rand::random::<f64>()) * self.j
                        + self.k
                })
                .take(samples)
                .map(|direction| scene.trace(Ray::new(self.eye, direction), max_bounces))
                .reduce(|accum, color| accum.map2(&color, |x, y| x + y))
                .expect("You must have at least one sample!")
                .map(|x| (x / samples as f32).sqrt());
            });
        image
    }
}
