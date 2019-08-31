use crate::material::Material;
use crate::ray::Ray;
use nalgebra::Point3;
use std::f64;


#[derive(Copy, Clone)]
pub struct Sphere {
    index: usize,
    center: Point3<f64>,
    radius: f64,
    material: Material,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Sphere {
    pub fn new(index: usize, center: Point3<f64>, radius: f64, material: Material) -> Self {
        Sphere {
            index,
            center,
            radius,
            material,
        }
    }

    // this only really needs to be exposed for bounding sphers
    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn center(&self) -> Point3<f64> {
        self.center
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn intersection_distance(&self, ray: &Ray) -> f64 {
        let bias = 1e-6;
        let op = self.center - ray.origin;
        let b = op.dot(&ray.direction);
        let det = b * b - op.dot(&op) + self.radius * self.radius;
        if det < 0f64 {
            return f64::INFINITY;
        }

        let det_root = det.sqrt();
        let t1 = b - det_root;
        if t1 > bias {
            return t1;
        }

        let t2 = b + det_root;
        if t2 > bias {
            return t2;
        }

        f64::INFINITY
    }
}
