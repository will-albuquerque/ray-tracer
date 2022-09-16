#![feature(array_zip)]

pub use camera::Camera;
pub use point::Point;
pub use ray::Ray;
pub use scene::Scene;
pub use sphere::Sphere;
pub use vector::Vector;

mod camera;
mod point;
mod ray;
mod scene;
mod sphere;
mod vector;
