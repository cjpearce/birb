use std::f32;
use nalgebra::Point3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
    material: Material
}

impl Sphere {
    pub fn new(center: Point3<f32>, radius: f32, material: Material) -> Self {
        Sphere{ center, radius, material }
    }

    pub fn center(&self) -> Point3<f32> {
        self.center
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn intersection_distance(&self, ray: &Ray) -> f32 {
        let bias = 1e-6;
        let op = self.center - ray.origin;
        let b = op.dot(&ray.direction);
        let det = b * b - op.dot(&op) + self.radius * self.radius;
        if det < 0f32 {
            return f32::INFINITY;
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

        f32::INFINITY
    }
}