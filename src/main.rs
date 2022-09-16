use std::f64::consts::PI;

use image::{DynamicImage, Rgb};
use ray_tracer::{Camera, Point, Scene, Sphere, Vector};

fn main() {
    let a = Sphere::new(Point::new([3.0, 0.0, 0.0]), 1.0, Rgb([0.5; 3]));
    let b = Sphere::new(Point::new([3.0, -101.0, 0.0]), 100.0, Rgb([0.5; 3]));
    let scene = Scene::new(&[a, b]);
    let camera = Camera::new(
        500,
        500,
        PI / 2.0,
        Point::new([1.0; 3]),
        Point::new([3.0, 0.0, 0.0]),
        Vector::new([0.0, 1.0, 0.0]),
    );
    DynamicImage::ImageRgb32F(camera.render(&scene, 500, 10))
        .to_rgb8()
        .save("test.png")
        .unwrap();
}
