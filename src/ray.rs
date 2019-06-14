use nalgebra::{Vector3, Point3};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}